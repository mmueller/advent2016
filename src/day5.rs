use crypto::md5::Md5;
use crypto::digest::Digest;
use errors::*;
use std::u64;
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;
use std::io::stdout;
use rand;
use rand::Rng;

const PASSWORD_LENGTH: usize = 8;
const PREFIX: &'static str = "ffykfhsq";

fn md5(index: u64, output: &mut [u8; 16]) {
    let mut hasher = Md5::new();
    hasher.input(PREFIX.as_bytes());
    hasher.input(index.to_string().as_bytes());
    hasher.result(output);
}

fn starts_with_five_zeroes(md5sum: &[u8; 16]) -> bool {
    md5sum[0] == 0 && md5sum[1] == 0 && md5sum[2] >> 4 == 0
}

fn as_hex(md5sum: &[u8; 16]) -> String {
    let mut s = String::new();
    for &byte in md5sum {
        write!(&mut s, "{:0>2x}", byte).unwrap();
    }
    s
}

fn part1() -> String {
    let mut password: Vec<char> = Vec::new();
    for i in 0..u64::MAX {
        let mut md5sum = [0; 16];
        md5(i, &mut md5sum);
        if starts_with_five_zeroes(&md5sum) {
            let md5hex = as_hex(&md5sum);
            password.push(md5hex.chars().nth(5).unwrap());
            print!("\rPassword so far: {}",
                     password.iter().cloned().collect::<String>());
            let _ = stdout().flush();
            if password.len() == PASSWORD_LENGTH {
                println!("");
                break;
            }
        }
    }
    password.into_iter().collect::<String>()
}

// Specific to part 2:
fn display_animated(password: &Vec<char>) {
    let mut rng = rand::thread_rng();
    print!("\rHACK THE PLANET: {}",
            password.iter().cloned()
                    .map(|c|
                        if c == '_' {
                            rng.gen_ascii_chars().next().unwrap()
                        } else {
                            c
                        })
                    .collect::<String>());
    let _ = stdout().flush();
}

fn part2() -> String {
    let mut password: Vec<char> = vec!['_'; 8];
    for i in 0..u64::MAX {
        let mut md5sum = [0; 16];
        md5(i, &mut md5sum);
        if starts_with_five_zeroes(&md5sum) {
            let md5hex = as_hex(&md5sum);
            let mut result_iter = md5hex.chars();
            let pos = result_iter.nth(5).unwrap();
            if pos >= '0' && pos < '8' {
                // Awkward but fast: convert ASCII digit to a usize
                let pos = pos as usize - 48;
                let val = result_iter.next().unwrap();
                if password[pos] == '_' {
                    *(password.get_mut(pos).unwrap()) = val;
                    display_animated(&password);
                    if password.iter().all(|c| *c != '_') {
                        println!("");
                        break;
                    }
                }
            }
        }
        if i % 250 == 0 {
            display_animated(&password);
        }
    }
    password.into_iter().collect::<String>()
}

pub fn day5() -> Result<()> {
    println!("PART 1:");
    part1();
    println!("\nPART 2:");
    part2();
    Ok(())
}
