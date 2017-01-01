use common;
use errors::*;

use std::fmt;
use regex::{Captures, Regex};

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

struct Display {
    pub width: usize,
    pub height: usize,
    rows: Vec<Vec<bool>>,
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               self.rows.iter()
                        .map(|row| {
                            row.iter()
                                .map(|&pixel| if pixel { '#' } else { '.' })
                                .collect::<String>()
                            })
                        .collect::<Vec<_>>()
                        .join("\n"))
    }
}

impl Display {
    pub fn new(width: usize, height: usize) -> Display {
        let mut rows: Vec<Vec<bool>> = Vec::new();
        for _ in 0..height {
            rows.push(vec![false; width]);
        }
        Display {
            width: width,
            height: height,
            rows: rows
        }
    }

    pub fn rect(&mut self, width: usize, height: usize) {
        for i in 0..height {
            for j in 0..width {
                self.rows[i][j] = true;
            }
        }
    }

    pub fn rotate_col(&mut self, x: usize, amount: usize) {
        let mut new_col: Vec<bool> = vec![false; self.height];
        for i in 0..self.height {
            new_col[(i+amount)%self.height] = self.rows[i][x];
        }
        for i in 0..self.height {
            self.rows[i][x] = new_col[i];
        }
    }

    pub fn rotate_row(&mut self, y: usize, amount: usize) {
        let mut new_row: Vec<bool> = vec![false; self.width];
        for i in 0..self.width {
            new_row[(i+amount)%self.width] = self.rows[y][i];
        }
        for i in 0..self.width {
            self.rows[y][i] = new_row[i];
        }
    }

    pub fn lit_pixels(&self) -> usize {
        self.rows.iter()
                 .map(|row| row.iter().filter(|&pixel| *pixel).count())
                 .fold(0, |total, count| total + count)
    }
}

// Retrieve a usize field from a regex (panics if not valid usize value)
fn usize_field(cap: &Captures, name: &str) -> usize {
    cap.name(name)
       .unwrap()
       .parse::<usize>()
       .unwrap()
}

pub fn day8() -> Result<()> {
    let re = Regex::new(r"(?P<cmd>rect|rotate) (?P<params>.*)")
                   .chain_err(|| "failed compiling regex")?;
    let rect_re = Regex::new(r"(?P<width>\d+)x(?P<height>\d+)")
                        .chain_err(|| "failed compiling regex")?;
    let rotate_re =
        Regex::new(r"(row y=(?P<y>\d+)|column x=(?P<x>\d+)) by (?P<amount>\d+)")
                   .chain_err(|| "failed compiling regex")?;

    // Gather instructions from input file
    let input = &mut String::new();
    common::read_file_to_string("input/day8.txt", input)?;

    let mut display = Display::new(WIDTH, HEIGHT);
    for line in input.lines() {
        match re.captures(line) {
            Some(captures) => {
                let params = captures.name("params").unwrap();
                match captures.name("cmd") {
                    Some("rect") => {
                        match rect_re.captures(params) {
                            Some(params) => {
                                display.rect(usize_field(&params, "width"),
                                             usize_field(&params, "height"));
                            },
                            None => { println!("rect parse error: {}", line) },
                        }
                    },
                    Some("rotate") => {
                        match rotate_re.captures(params) {
                            Some(params) => {
                                if params.name("x").is_some() {
                                    display.rotate_col(
                                        usize_field(&params, "x"),
                                        usize_field(&params, "amount"));
                                } else if params.name("y").is_some() {
                                    display.rotate_row(
                                        usize_field(&params, "y"),
                                        usize_field(&params, "amount"));
                                } else {
                                    println!("didn't understand: {}", line);
                                }
                            },
                            None => { println!("rotate parse error: {}", line) }
                        }
                    },
                    Some(_) => { println!("parse error: {}", line) },
                    None => { println!("parse error: {}", line) },
                }
            },
            None => {
                println!("Couldn't parse: {}", line);
            },
        }
    }

    // Report results
    println!("{}", display);
    println!("Lit pixels: {}", display.lit_pixels());
    Ok(())
}
