use std::collections::HashMap;

use common;
use errors::*;

fn collect_message(counts: Vec<HashMap<char, usize>>,
                   most_common: bool) -> String {
    counts.iter()
          .map(|count_map| {
              count_map.iter()
                       .max_by_key(|&(_, count)|
                           if most_common {
                               *count as isize
                           } else {
                               -(*count as isize)
                           })
                       .unwrap()
                       .0
          })
          .cloned()
          .collect::<String>()
}

pub fn day6() -> Result<()> {
    // Gather messages from input file
    let input = &mut String::new();
    common::read_file_to_string("input/day6.txt", input)?;
    let messages: Vec<&str> = input.trim().split("\n").collect();

    // Collect character statisticts for each position in message
    let mut counts: Vec<HashMap<char, usize>> = Vec::new();
    for message in &messages {
        for (i, c) in message.chars().enumerate() {
            if counts.len() == i {
                counts.push(HashMap::new());
            }
            let new_count: usize = *counts[i].entry(c).or_insert(0) + 1;
            counts[i].insert(c, new_count);
        }
    }

    // Report results
    println!("part 1: {}", collect_message(counts.clone(), true));
    println!("part 2: {}", collect_message(counts, false));
    Ok(())
}
