#[macro_use]
extern crate error_chain;
extern crate regex;

mod common;
mod day1;
mod errors;

fn main() {
    if let Err(ref e) = day1::day1() {
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
