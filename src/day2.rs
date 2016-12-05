use common;
use errors::*;
use std::ops::Index;

struct Keypad {
    pub rows: Vec<Vec<char>>,
}

impl Keypad {
    fn new() -> Keypad {
        Keypad { rows: Vec::new() }
    }

    fn add_row(&mut self, row: &[char]) -> &mut Keypad {
        self.rows.push(row.to_vec());
        self
    }

    fn execute_instructions(&self, instructions: Vec<&str>,
                            start_pos: (usize, usize)) -> String {
        let mut result = String::new();
        for line in instructions {
            let mut pos: (usize, usize) = start_pos;
            for char in line.chars() {
                let new_pos: (usize, usize) = match char {
                    'U' if pos.0 > 0 => (pos.0 - 1, pos.1),
                    'D' if pos.0 < self.rows.len() - 1 => (pos.0 + 1, pos.1),
                    'L' if pos.1 > 0 => (pos.0, pos.1 - 1),
                    'R' if pos.1 < self.rows[0].len() - 1 => (pos.0, pos.1 + 1),
                    _ => { pos },
                };
                if self[new_pos.0][new_pos.1] != 'x' {
                    pos = new_pos;
                }
            }
            result.push(self[pos.0][pos.1]);
        }
        result
    }
}

impl Index<usize> for Keypad {
    type Output = Vec<char>;

    fn index(&self, i: usize) -> &Vec<char> {
        &self.rows[i]
    }
}

pub fn day2() -> Result<()> {
    // Gather instructions from input file
    let input = &mut String::new();
    common::read_file_to_string("input/day2.txt", input)?;
    let instructions: Vec<&str> = input.split("\n")
                                       .filter(|s| !s.is_empty())
                                       .collect();

    // Use instructions on keypad1
    let ref mut keypad1 = Keypad::new();
    keypad1.add_row(&['1', '2', '3'])
           .add_row(&['4', '5', '6'])
           .add_row(&['7', '8', '9']);
    println!("keypad 1 code: {}.",
             keypad1.execute_instructions(instructions.clone(), (1, 1)));

    // Use instructions on keypad2
    let ref mut keypad2 = Keypad::new();
    keypad2.add_row(&['x', 'x', '1', 'x', 'x'])
           .add_row(&['x', '2', '3', '4', 'x'])
           .add_row(&['5', '6', '7', '8', '9'])
           .add_row(&['x', 'A', 'B', 'C', 'x'])
           .add_row(&['x', 'x', 'D', 'x', 'x']);
    println!("keypad 2 code: {}.",
             keypad2.execute_instructions(instructions.clone(), (2, 0)));
    Ok(())
}

