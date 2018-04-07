#[macro_use]
extern crate structopt;
extern crate zalgo;


use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process;

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

    /// Set mangling intensity
    #[structopt(short = "s", long = "intensity", parse(try_from_str = "parse_intensity"))]
    intensity: Option<Intensity>,

    /// Input text
    text: Option<String>,
}

fn parse_intensity(s: &str) -> Result<Intensity, String> {
    let intensity = match s {
        "mini" => Intensity::Mini,
        "normal" => Intensity::Normal,
        "maxi" => Intensity::Maxi,
        _ => return Err("fuuuuuu".into()),
    };

    Ok(intensity)
}


fn main() {
    let app = Args::clap();
    let matches = app.get_matches();
    let args = Args::from_clap(&matches);

    let combined_kind = {
        let mut k = CharKind::empty();

        if args.up     { k.insert(CharKind::UP)     }
        if args.middle { k.insert(CharKind::MIDDLE) }
        if args.down   { k.insert(CharKind::DOWN)   }

        if k.is_empty() {
            k = CharKind::MIDDLE | CharKind::DOWN;
        }

        k
    };

    let intensity = args.intensity.unwrap_or(Intensity::Mini);

    let text = if let Some(path) = args.input {
        let mut buf = String::new();

        let mut f = File::open(path).unwrap();
        f.read_to_string(&mut buf).unwrap();

        buf
    } else if let Some(text) = args.text {
        text
    } else {
        let stderr = io::stderr();
        let mut lock = stderr.lock();

        lock.write(matches.usage().as_bytes()).unwrap();
        lock.write(b"\n").unwrap();

        process::exit(1);
    };

    let mut output: Box<io::Write> = if let Some(path) = args.output {
        Box::new(File::create(path).unwrap())
    } else {
        Box::new(io::stdout())
    };

    let zalgoified_text = zalgo::apply(&text, combined_kind, intensity);

    output.write(zalgoified_text.as_bytes()).unwrap();
}
