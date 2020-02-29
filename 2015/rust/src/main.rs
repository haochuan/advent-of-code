extern crate day1;
extern crate day2;
extern crate day3;
extern crate day4;
extern crate day5;
extern crate day6;
// extern crate day7;
extern crate day8;
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
            1 => day1::run(),
            2 => day2::run(),
            3 => day3::run(),
            4 => day4::run(),
            5 => day5::run(),
            6 => day6::run(),
            // 7 => day7::run(),
            8 => day8::run(),
            _ => println!("Day {} is not valid, or it has not been solved.", day),
        }
    }
}
