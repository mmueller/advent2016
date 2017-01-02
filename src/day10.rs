use common;
use errors::*;
use regex::Regex;
use std::fmt;

lazy_static! {
    static ref INSTRUCTION_RE: Regex =
        Regex::new(concat!(
            r"bot (?P<id>\d+) gives ",
            r"low to (?P<desttype1>bot|output) (?P<destid1>\d+) and ",
            r"high to (?P<desttype2>bot|output) (?P<destid2>\d+)")).unwrap();
}

#[derive(Clone,Copy)]
enum Destination {
    Nowhere,
    Bot(usize),
    Output(usize)
}

#[derive(Clone,Copy)]
struct Bot {
    pub id: usize,
    pub low: Option<usize>,
    pub high: Option<usize>,
    pub low_dest: Destination,
    pub high_dest: Destination,
}

impl Bot {
    fn new(id: usize) -> Bot {
        Bot {
            id: id,
            low: None,
            high: None,
            low_dest: Destination::Nowhere,
            high_dest: Destination::Nowhere
        }
    }

    fn take_value(&mut self, value: usize) -> Result<()> {
        if self.low == None {
            self.low = Some(value);
            Ok(())
        } else if self.high == None {
            if value > self.low.unwrap() {
                self.high = Some(value);
            } else {
                self.high = self.low;
                self.low = Some(value);
            }
            Ok(())
        } else {
            Err(format!("bot {} has two values, can't take {}",
                        self.id, value).into())
        }
    }

    fn take_instruction(&mut self, instruction: &str) -> Result<()> {
        match INSTRUCTION_RE.captures(instruction) {
            Some(captures) => {
                let id = captures.name("id").unwrap().parse::<usize>().unwrap();
                if id != self.id {
                    return Err(format!("wasn't for me! {} != {}",
                                       id, self.id).into())
                }
                let destid1 = captures.name("destid1").unwrap()
                                      .parse::<usize>().unwrap();
                let destid2 = captures.name("destid2").unwrap()
                                      .parse::<usize>().unwrap();
                match captures.name("desttype1") {
                    Some("bot") => {
                        self.low_dest = Destination::Bot(destid1);
                    },
                    Some("output") => {
                        self.low_dest = Destination::Output(destid1);
                    },
                    Some(_) => { unreachable!() },
                    None => { unreachable!() }
                }
                match captures.name("desttype2") {
                    Some("bot") => {
                        self.high_dest = Destination::Bot(destid2);
                    },
                    Some("output") => {
                        self.high_dest = Destination::Output(destid2);
                    },
                    Some(_) => { unreachable!() },
                    None => { unreachable!() }
                }
                Ok(())
            },
            None => {
                Err(format!("bad instruction: {}", instruction).into())
            }
        }
    }
}

impl fmt::Display for Bot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Bot {}: {}, {}>",
               self.id,
               if self.low.is_some() { self.low.unwrap().to_string() }
                                else { "()".to_string() },
               if self.high.is_some() { self.high.unwrap().to_string() }
                                 else { "()".to_string() })
    }
}

fn ensure_bot_exists(bots: &mut Vec<Bot>, id: usize) {
    for i in bots.len()..(id+1) {
        bots.push(Bot::new(i));
    }
}

fn ensure_output_exists(outputs: &mut Vec<Vec<usize>>, id: usize) {
    for _ in outputs.len()..(id+1) {
        outputs.push(Vec::new());
    }
}

pub fn day10() -> Result<()> {
    let value_re = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
    let input = &mut String::new();
    common::read_file_to_string("input/day10.txt", input)?;

    let mut bots: Vec<Bot> = Vec::new();

    for line in input.lines() {
        match value_re.captures(line) {
            Some(captures) => {
                let value = captures.at(1).unwrap().parse::<usize>().unwrap();
                let id = captures.at(2).unwrap().parse::<usize>().unwrap();
                ensure_bot_exists(&mut bots, id);
                bots[id].take_value(value)?;
            },
            None => {
                match INSTRUCTION_RE.captures(line) {
                    Some(captures) => {
                        let id = captures.name("id").unwrap()
                                         .parse::<usize>().unwrap();
                        ensure_bot_exists(&mut bots, id);
                        bots[id].take_instruction(line)?;
                    },
                    None => {
                        println!("bad instruction: {}", line);
                    }
                }
            }
        }
    }

    let mut outputs: Vec<Vec<usize>> = Vec::new();

    loop {
        let clone: Bot = {
            let mut next_bot: Option<&mut Bot> = None;

            // Find the next bot that has an instruction to execute
            for bot in bots.iter_mut() {
                // Part 1: If the bot has chips 17 and 61, print it out.
                if bot.low == Some(17) && bot.high == Some(61) {
                    println!("Found bot: {}", bot);
                }
                if bot.low.is_some() && bot.high.is_some() {
                    match bot.low_dest {
                        Destination::Bot(_) => {
                            next_bot = Some(bot);
                            break;
                        }
                        Destination::Output(_) => {
                            next_bot = Some(bot);
                            break;
                        },
                        Destination::Nowhere => { }
                    }
                }
            }
    
            if next_bot.is_none() {
                println!("Nothing left to do!");
                break;
            }

            // Save a copy of the bot for later execution
            let bot = next_bot.unwrap();
            let clone = bot.clone();

            // Empty this bot's hands
            bot.low = None;
            bot.high = None;

            clone
        };

        // Execute the clone's commands
        match clone.low_dest {
            Destination::Bot(destid) => {
                bots[destid].take_value(clone.low.unwrap())?;
            },
            Destination::Output(destid) => {
                ensure_output_exists(&mut outputs, destid);
                outputs[destid].push(clone.low.unwrap());
            },
            Destination::Nowhere => { unreachable!() }
        }
        
        match clone.high_dest {
            Destination::Bot(destid) => {
                bots[destid].take_value(clone.high.unwrap())?;
            },
            Destination::Output(destid) => {
                ensure_output_exists(&mut outputs, destid);
                outputs[destid].push(clone.high.unwrap());
            },
            Destination::Nowhere => { unreachable!() }
        }
    }

    // Part 2: Dump contents of outputs so we can see what's in 0, 1, and 2.
    println!("Outputs:");
    for (i, output) in outputs.iter().enumerate() {
        print!("{}: ", i);
        for value in output {
            print!("{} ", value);
        }
        println!("");
    }

    Ok(())
}
