extern crate colored;
extern crate rand;
#[macro_use]
extern crate structopt;
extern crate zalgo;


use std::ffi::OsStr;
use std::fmt;
use std::fs::File;
use std::io::{self, Read, Write};
use std::num;
use std::path::PathBuf;
use std::process;
use std::str;

use colored::Colorize;
use rand::{jitter, Rng};
use structopt::StructOpt;
use zalgo::{CharKind, Intensity};


#[derive(Debug, StructOpt)]
struct Args {
    /// Read from specified input (default: stdin)
    #[structopt(short = "i", long = "input", parse(from_os_str = "parse_input"))]
    input: Option<Input>,

    /// Write to specified output (default: stdout)
    #[structopt(short = "o", long = "output", parse(from_os_str = "parse_output"))]
    output: Option<Output>,

    /// Random number generator,
    /// allowed values: chacha, isaac, isaac64, jitter, os, std, thread, xorshift (default: thread)
    #[structopt(short = "r", long = "rng")]
    rng: Option<String>,

    /// Enable up chars
    #[structopt(short = "u", long = "up")]
    up: bool,

    /// Enable middle chars
    #[structopt(short = "m", long = "middle")]
    middle: bool,

    /// Enable down chars
    #[structopt(short = "d", long = "down")]
    down: bool,

    /// Set mangling intensity,
    /// allowed values: mini, normal, maxi, random, custom(<up>,<middle>,<down>) (default: mini)
    #[structopt(short = "s", long = "intensity", parse(try_from_str = "parse_intensity"))]
    intensity: Option<Intensity>,

    /// Input text
    text: Option<String>,
}

#[derive(Debug)]
enum Input {
    Stdin,
    Path(PathBuf),
}

fn parse_input(s: &&OsStr) -> Input {
    if s.to_str() == Some("-") {
        Input::Stdin
    } else {
        Input::Path(PathBuf::from(s))
    }
}

#[derive(Debug)]
enum Output {
    Stdout,
    Path(PathBuf),
}

fn parse_output(s: &&OsStr) -> Output {
    if s.to_str() == Some("-") {
        Output::Stdout
    } else {
        Output::Path(PathBuf::from(s))
    }
}

fn parse_rng(s: &str) -> Result<Box<Rng>, Error> {
    let rng: Box<Rng> = match s {
        "chacha" => Box::new(rand::ChaChaRng::new_unseeded()),
        "isaac" => Box::new(rand::IsaacRng::new_unseeded()),
        "isaac64" =>  Box::new(rand::Isaac64Rng::new_unseeded()),
        "jitter" => Box::new(rand::JitterRng::new()?),
        "os" => Box::new(rand::OsRng::new()?),
        "std" => Box::new(rand::StdRng::new()?),
        "thread" => Box::new(rand::thread_rng()),
        "xorshift" =>  Box::new(rand::XorShiftRng::new_unseeded()),

        _ => return Err(Error::ParseRng(s.into())),
    };

    Ok(rng)
}

fn parse_intensity(s: &str) -> Result<Intensity, Error> {
    let intensity = match s {
        "mini" => Intensity::Mini,
        "normal" => Intensity::Normal,
        "maxi" => Intensity::Maxi,
        "random" => Intensity::Random,
        s if s.starts_with("custom(") && s.ends_with(")") => {
            let mut params = s[7..s.len()-1].split(',');

            let parse_param = |params: &mut str::Split<char>| -> Result<_, Error> {
                let str_param = params
                    .next()
                    .ok_or_else(|| Error::ParseIntensity(s.into()))?
                    .trim();

                let param = str_param
                    .parse::<usize>()
                    .map_err(|e| Error::ParseInt(str_param.into(), e))?;

                Ok(param)
            };

            let up = parse_param(&mut params)?;
            let middle = parse_param(&mut params)?;
            let down = parse_param(&mut params)?;

            if params.next().is_some() {
                return Err(Error::ParseIntensity(s.into()));
            }

            Intensity::Custom { up, middle, down }
        },

        _ => return Err(Error::ParseIntensity(s.into())),
    };

    Ok(intensity)
}


#[derive(Debug)]
enum Error {
    ParseIntensity(String),
    ParseInt(String, num::ParseIntError),
    ParseRng(String),
    OpenFile(PathBuf, io::Error),
    CreateFile(PathBuf, io::Error),
    Io(io::Error),
    JitterTimer(jitter::TimerError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ParseIntensity(ref string) => {
                write!(f, "Couldn't parse {:?} as an intensity parameter", string)
            },
            Error::ParseInt(ref str_param, ref pie) => {
                write!(f, "Couldn't parse {:?} as an integer: {}", str_param, pie)
            },
            Error::ParseRng(ref string) => {
                write!(f, "Couldn't parse {:?} as an RNG parameter", string)
            },
            Error::OpenFile(ref path, ref ioe) => {
                write!(f, "Error when opening file `{}`: {}.", path.display(), ioe)
            },
            Error::CreateFile(ref path, ref ioe) => {
                write!(f, "Error when creating file `{}`: {}.", path.display(), ioe)
            },
            Error::Io(ref ioe) => {
                write!(f, "IO error: {}", ioe)
            },
            Error::JitterTimer(ref jte) => {
                write!(f, "Jitter timer error: {}", jte)
            }
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<jitter::TimerError> for Error {
    fn from(e: jitter::TimerError) -> Error {
        Error::JitterTimer(e)
    }
}


fn run(args: Args) -> Result<(), Error> {
    let rng = args.rng.map(|s| parse_rng(&s)).unwrap_or_else(|| Ok(Box::new(rand::thread_rng())))?;

    let combined_kind = {
        let mut k = CharKind::empty();

        if args.up     { k.insert(CharKind::UP)     }
        if args.middle { k.insert(CharKind::MIDDLE) }
        if args.down   { k.insert(CharKind::DOWN)   }

        // This is the default when provided no flags for kinds
        if k.is_empty() {
            k = CharKind::MIDDLE | CharKind::DOWN;
        }

        k
    };

    let intensity = args.intensity.unwrap_or(Intensity::Mini);

    let read_from_stdin = || -> Result<_, Error> {
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        let mut buf = String::new();
        handle.read_to_string(&mut buf)?;

        Ok(buf)
    };

    // 1) Prefer reading from explicitly specified input first
    // 2) Then if no input specified, read from argument list
    // 3) If there are no positional arguments too, read from stdin
    let text = if let Some(input) = args.input {
        match input {
            Input::Stdin => read_from_stdin()?,
            Input::Path(path) => {
                let mut buf = String::new();

                let mut f = File::open(&path).map_err(|e| Error::OpenFile(path, e))?;
                f.read_to_string(&mut buf)?;

                buf
            },
        }
    } else if let Some(text) = args.text {
        text
    } else {
        read_from_stdin()?
    };

    let write_to_output = |output: &mut Write| -> Result<_, Error> {
        // This way we don't have to spend time allocating a String for the whole
        // Zalgo text and wasting memory in the process.
        for c in zalgo::apply_rng_iter(rng, text.chars(), combined_kind, intensity) {
            let mut buf = [0u8; 4];
            let s = c.encode_utf8(&mut buf);
            output.write_all(s.as_bytes())?;
        }

        output.write_all(b"\n")?;
        output.flush()?;

        Ok(())
    };

    if let Some(Output::Path(path)) = args.output {
        let mut f = File::create(&path).map_err(|e| Error::CreateFile(path, e))?;
        write_to_output(&mut f)?;
    } else {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        write_to_output(&mut handle)?;
    };

    Ok(())
}

fn main() {
    let app = Args::clap();
    let matches = app.get_matches();
    let args = Args::from_clap(&matches);

    if let Err(e) = run(args) {
        eprintln!("{} {}", "error:".bold().red(), e);
        eprintln!("{}", matches.usage());

        process::exit(1);
    }
}
