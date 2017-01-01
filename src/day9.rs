use common;
use errors::*;
use regex::Regex;

// Doesn't actually return data, just returns the size of the output.
fn decompressed_size(input: &str, recurse: bool) -> Result<usize> {
    let re = Regex::new(r"^\((\d+)x(\d+)\)")
                   .chain_err(|| "failed compiling regex")?;
    let mut output_size: usize = 0;
    let mut current_input = input;

    loop {
        match re.captures(current_input) {
            Some(captures) => {
                let length = captures.at(1).unwrap().parse::<usize>().unwrap();
                let times = captures.at(2).unwrap().parse::<usize>().unwrap();
                let metadata_pos = captures.pos(0).unwrap();
                let next_parts = current_input.split_at(metadata_pos.1).1
                                              .split_at(length);
                // The uncompressed portion before the next metadata instruction
                output_size += current_input.split_at(metadata_pos.0).0.len();
                // The repeated part
                output_size += if recurse {
                    match decompressed_size(next_parts.0, true) {
                        Ok(repeated_length) => {
                            repeated_length * times
                        },
                        Err(e) => {
                            return Err(e)
                        }
                    }
                } else {
                    next_parts.0.len() * times
                };
                current_input = next_parts.1;
            },
            None => {
                // Copy remainder of the input
                output_size += current_input.len();
                break;
            }
        }
    }

    Ok(output_size)
}

pub fn day9() -> Result<()> {
    let input = &mut String::new();
    common::read_file_to_string("input/day9.txt", input)?;

    match decompressed_size(input.trim(), false) {
        Ok(size) => { println!("Decompressed length (pt 1): {}", size) }
        Err(e) => { println!("Failed to decompress: {}", e) }
    }

    match decompressed_size(input.trim(), true) {
        Ok(size) => { println!("Decompressed length (pt 2): {}", size) }
        Err(e) => { println!("Failed to decompress: {}", e) }
    }

    Ok(())
}
