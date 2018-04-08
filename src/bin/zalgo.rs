extern crate colored;
#[macro_use]
extern crate structopt;
extern crate zalgo;


use std::fmt;
use std::fs::File;
use std::io::{self, Read, Write};
use std::num;
use std::path::PathBuf;
use std::process;
use std::str;

use colored::Colorize;
use structopt::StructOpt;
use zalgo::{CharKind, Intensity};


#[derive(Debug, StructOpt)]
struct Args {
    /// Read from input file instead of stdin
    #[structopt(short = "i", help = "Input file", parse(from_os_str))]
    input: Option<PathBuf>,

    /// Write to output file instead of stdout
    #[structopt(short = "o", help = "Output file", parse(from_os_str))]
    output: Option<PathBuf>,

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
    /// allowed values: mini, normal, maxi, random, custom(<up>,<middle>,<down>)
    #[structopt(short = "s", long = "intensity", parse(try_from_str = "parse_intensity"))]
    intensity: Option<Intensity>,

    /// Input text
    text: Option<String>,
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
    OpenFile(PathBuf, io::Error),
    CreateFile(PathBuf, io::Error),
    Io(io::Error),
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
            Error::OpenFile(ref path, ref ioe) => {
                write!(f, "Error when opening file `{}`: {}.", path.display(), ioe)
            },
            Error::CreateFile(ref path, ref ioe) => {
                write!(f, "Error when creating file `{}`: {}.", path.display(), ioe)
            },
            Error::Io(ref ioe) => {
                write!(f, "IO error: {}", ioe)
            },
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}


fn run(args: Args) -> Result<(), Error> {
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

    let text = if let Some(path) = args.input {
        // Prefer reading from files first

        let mut buf = String::new();

        let mut f = File::open(&path).map_err(|e| Error::OpenFile(path, e))?;
        f.read_to_string(&mut buf)?;

        buf
    } else if let Some(text) = args.text {
        // Then if no input file specified, read from argument list

        text
    } else {
        // If there are no positional arguments too, read from stdin

        let stdin = io::stdin();
        let mut handle = stdin.lock();

        let mut buf = String::new();
        handle.read_to_string(&mut buf)?;

        buf
    };

    let mut output: Box<io::Write> = if let Some(path) = args.output {
        Box::new(File::create(&path).map_err(|e| Error::CreateFile(path, e))?)
    } else {
        Box::new(io::stdout())
    };

    let zalgoified_text = zalgo::apply(&text, combined_kind, intensity);

    writeln!(output, "{}", zalgoified_text)?;

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
