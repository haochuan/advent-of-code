use advent2019::{self, bail, Error, Reader};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::exit;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Number of day
    day: usize,

    /// Optional input file, will use stdin if there is nothing supplied
    input: Option<PathBuf>,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        let mut e: &dyn std::error::Error = &e;
        while let Some(source) = e.source() {
            eprintln!("    - caused by: {}", source);
            e = source;
        }
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    let opt = Opt::from_args();
    let stdin = io::stdin();
    let input = match opt.input {
        Some(path) => {
            let file = fs::File::open(path).unwrap();
            let reader = io::BufReader::new(file);
            Reader::File(reader)
        }
        None => {
            let guard = stdin.lock();
            Reader::Stdin(guard)
        }
    };
    match opt.day {
        1 => advent2019::day01::run(input)?,
        2 => advent2019::day02::run(input)?,
        3 => advent2019::day03::run(input)?,
        4 => advent2019::day04::run(input)?,
        5 => advent2019::day05::run(input)?,
        n if n > 1 && n < 26 => bail!("Day {} has not been implemented yet.", n),
        _ => bail!("Day should be the range of 1 to 25."),
    }
    Ok(())
}
