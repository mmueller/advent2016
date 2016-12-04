#[macro_use]
extern crate error_chain;
extern crate regex;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! { }
}

use errors::*;

use std::collections::HashSet;
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
    fn turn_right(&self) -> Direction {
        match *self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }
    fn go(&mut self, d: Direction, dist: i32) {
        match d {
            Direction::North => self.y -= dist,
            Direction::East  => self.x += dist,
            Direction::South => self.y += dist,
            Direction::West  => self.x -= dist,
        }
    }
    fn manhattan_distance(&self, other: Point) -> i32 {
         (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn day1() -> Result<()> {
    // Gather instructions from input file
    let re = Regex::new(r"^([LR])(\d+)$")
                   .chain_err(|| "failed compiling regex")?;
    let mut f = File::open("input/day1.txt")
                     .chain_err(|| "failed to open day 1 input file")?;
    let mut s = String::new();
    f.read_to_string(&mut s)
     .chain_err(|| "failed reading day 1 input file")?;
    let instructions : Vec<&str> = s.trim().split(", ").collect();

    // Execute instructions
    let mut visited = HashSet::new();
    let mut current_position = Point::origin();
    let mut direction = Direction::North;
    let mut point_visited_twice = Point::origin();
    for instruction in &instructions {
        for cap in re.captures_iter(instruction) {
            direction = match cap.at(1).unwrap() {
                "L" => direction.turn_left(),
                "R" => direction.turn_right(),
                _ => bail!("unexpected"),
            };
            let dist = cap.at(2).unwrap()
                          .parse::<i32>()
                          .chain_err(|| format!("parse fail {}", instruction))?;
            for _ in 0..dist {
                current_position.go(direction, 1);
                if point_visited_twice == Point::origin() {
                    if visited.contains(&current_position) {
                        point_visited_twice = current_position.clone();
                    } else {
                        visited.insert(current_position);
                    }
                }
            }
        }
    }

    // Report results
    println!("Final location: {}, {}",
             current_position.x, current_position.y);
    println!("Total blocks: {}",
             current_position.manhattan_distance(Point::origin()));
    println!("");
    println!("First point visited twice: {}, {}",
             point_visited_twice.x, point_visited_twice.y);
    println!("Blocks to this point: {}",
             point_visited_twice.manhattan_distance(Point::origin()));
    Ok(())
}

fn main() {
    if let Err(ref e) = day1() {
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
