// extern crate day1;
extern crate day2;
//
extern crate clap;
use clap::{crate_version, App, Arg};

fn main() {
    let matches = App::new("aoc")
        .version(crate_version!())
        .author("Haochuan Liu <haochuan.liu@gmail.com>")
        .about("Advent of Code 2015 Rust Solution CLI")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("DAY")
                .help("Get the solution for day X")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    if let Some(d) = matches.value_of("day") {
        let day = d.parse::<u8>().unwrap();
        match day {
            // 1 => day1::run(),
            2 => day2::run(),
            _ => println!("Day {} is not valid, or it has not been solved.", day),
        }
    }
}