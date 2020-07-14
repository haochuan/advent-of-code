use advent2019;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Number of day
    day: usize,

    /// Optional input file, will use stdin if there is nothing supplied
    input: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    advent2019::day01::run();
}
