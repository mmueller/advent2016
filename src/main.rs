extern crate argparse;
extern crate crypto;
#[macro_use]
extern crate error_chain;
extern crate rand;
extern crate regex;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod errors;

use argparse::{ArgumentParser, StoreOption};

fn main() {
    let mut day: Option<u32> = None;
    let mut parser = ArgumentParser::new();
    parser.set_description("Advent of Code 2016");
    parser.refer(&mut day)
          .add_option(&["-d", "--day"], StoreOption,
                      "number of challenge to run");
    parser.parse_args_or_exit();
    let result = match day {
        Some(1) => day1::day1(),
        Some(2) => day2::day2(),
        Some(3) => day3::day3(),
        Some(4) => day4::day4(),
        Some(5) => day5::day5(),
        Some(_) => { println!("Invalid day specified."); Ok(()) },
        None => { println!("--day is required."); Ok(()) },
    };
    if let Err(ref e) = result {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}
