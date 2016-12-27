use std::collections::VecDeque;
use regex::Regex;

use common;
use errors::*;

lazy_static! {
    static ref HYPER_RE: Regex = Regex::new(r"\[([^\]]+)\]").unwrap();
}

fn is_abba(window: &VecDeque<char>) -> bool {
    window.len() == 4 &&
        window[0] != window[1] &&
        window[0] == window[3] &&
        window[1] == window[2]
}

fn supports_tls(address: &&str) -> bool {
    let mut window : VecDeque<char> = VecDeque::new();
    let mut in_brackets = false;
    let mut result = false;

    for c in (*address).chars() {
        match c {
            '[' => {
                in_brackets = true;
                window.clear();
            },
            ']' => {
                in_brackets = false;
                window.clear();
            }
            c => {
                window.push_back(c);
                if window.len() > 4 {
                    window.pop_front();
                }
                if is_abba(&window) {
                    if in_brackets {
                        return false;
                    }
                    result = true;
                }
            }
        }
    }
    result
}

fn find_abas(address: &str) -> Vec<(char, char)> {
    let mut window : VecDeque<char> = VecDeque::new();
    let mut in_brackets = false;
    let mut result: Vec<(char, char)> = Vec::new();

    for c in (*address).chars() {
        match c {
            '[' => {
                in_brackets = true;
                window.clear();
            },
            ']' => {
                in_brackets = false;
                window.clear();
            },
            c => {
                if !in_brackets {
                    window.push_back(c);
                    if window.len() > 3 {
                        window.pop_front();
                    }
                    if window.len() == 3 &&
                       window[0] != window[1] && window[0] == window[2] {
                        result.push((window[0], window[1]));
                    }
                }
            }
        }
    }
    result
}

fn supports_ssl(address: &&str) -> bool {
    for aba in find_abas(*address) {
        let bab: String = vec![aba.1, aba.0, aba.1].into_iter().collect();
        for hyperwhatever in HYPER_RE.captures_iter(*address) {
            if hyperwhatever.at(1).unwrap().contains(&bab) {
                return true;
            }
        }
    }
    false
}

pub fn day7() -> Result<()> {
    // Gather addresses from input file
    let input = &mut String::new();
    common::read_file_to_string("input/day7.txt", input)?;
    let addresses: Vec<&str> = input.trim().split("\n").collect();

    // Filter addresses
    let num_tls_addresses = addresses.iter()
                                     .cloned()
                                     .filter(supports_tls)
                                     .count();
    let num_ssl_addresses = addresses.iter()
                                     .cloned()
                                     .filter(supports_ssl)
                                     .count();

    // Report results
    println!("TLS: {}", num_tls_addresses);
    println!("SSL: {}", num_ssl_addresses);
    Ok(())
}
