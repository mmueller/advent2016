use common;
use errors::*;
use regex::Regex;
use std::collections::{BTreeMap, HashMap};
use std::char;
use std::str;

const CHECKSUM_LEN: usize = 5;

fn get_checksum(encrypted_name: &str) -> String {
    // God this feels disgusting.
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    for char in encrypted_name.chars() {
        if char == '-' {
            continue;
        }
        if char_counts.contains_key(&char) {
            let new_value = char_counts[&char] + 1;
            char_counts.insert(char, new_value);
        } else {
            char_counts.insert(char, 1);
        }
    }
    // Create a representation sorted by count, so we can traverse in
    // descending order to build the checksum.
    let mut count_to_charset: BTreeMap<usize, String> = BTreeMap::new();
    for (char, count) in char_counts.iter() {
        if count_to_charset.contains_key(count) {
            count_to_charset.get_mut(count).unwrap().push(*char);
        } else {
            count_to_charset.insert(*count, (*char).to_string().clone());
        }
    }

    count_to_charset.iter()
                    .rev()
                    .fold(Vec::new(), |checksum, (_, chars)| {
                        let mut sorted_charset: Vec<char> = chars.chars()
                                                                 .collect();
                        sorted_charset.sort();
                        [checksum, sorted_charset].concat()
                    })
                    .iter()
                    .take(CHECKSUM_LEN)
                    .cloned()
                    .collect::<String>()
}

// Rotates characters a-z by the given amount, other characters are untouched.
fn rotate_char(c: char, rotn: u8) -> char {
    if c >= 'a' && c <= 'z' {
        // These "unsafe" byte operations are actually safe since we know
        // we're in the range 'a' ... 'z'.
        let rotated: u8 = ((c as u8) - 97 + rotn) % 26 + 97;
        char::from_u32(rotated as u32)
             .unwrap()
    } else {
        c
    }
}

fn decrypt_name(encrypted_name: &str, sector_id: u32) -> String {
    let rotn: u8 = (sector_id % 26) as u8;
    encrypted_name.chars()
                  .map(|c| rotate_char(c, rotn))
                  .collect::<String>()
}

pub fn day4() -> Result<()> {
    // Gather instructions from input file
    let input = &mut String::new();
    common::read_file_to_string("input/day4.txt", input)?;
    let room_names: Vec<&str> = input.split("\n")
                                     .filter(|s| !s.is_empty())
                                     .collect();

    let re = Regex::new(r"^([-a-z]+)-(\d+)\[([a-z]{5})\]$")
                   .chain_err(|| "failed compiling regex")?;
    let mut sector_total : u32 = 0;
    for room_name in room_names {
        match re.captures(room_name) {
            Some(captures) => {
                let encrypted_name = captures.at(1).unwrap();
                let sector_id_opt = captures.at(2).unwrap().parse::<u32>();
                let checksum = captures.at(3).unwrap();
                match sector_id_opt {
                    Ok(sector_id) => {
                        if get_checksum(encrypted_name) == checksum {
                            sector_total += sector_id;
                            println!("Found room: {}: {}", sector_id,
                                     decrypt_name(encrypted_name, sector_id));
                        }
                    },
                    Err(_) => println!("Sector {} didn't parse.",
                                         captures.at(2).unwrap()),
                }

            },
            None => println!("Room {} doesn't match regex.", room_name),
        }
    }
    println!("Total of valid rooms: {}", sector_total);
    Ok(())
}
