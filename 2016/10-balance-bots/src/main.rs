use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;

enum Dest {
    Bot(usize),
    Out(usize),
}

struct Input {
    value: usize,
    destination: usize,
}

struct Bot {
    lo_dest: Dest,
    hi_dest: Dest,
    chips: Vec<usize>,
}


impl Bot {
    fn give(&mut self, chip: usize) {
        self.chips.push(chip);
        self.chips.sort();
    }

    fn ready(&self) -> bool { self.chips.len() == 2 }
}

struct BotArena {
    bots: Vec<Bot>,
    inputs: Vec<Input>,
    size: usize,
}

impl BotArena {
    fn new() -> BotArena {
        let mut arena = BotArena {
            bots: Vec::new(),
            inputs: Vec::new(),
            size: 0,
        };
        for _ in 0..256 {
            arena.bots.push(Bot {
                lo_dest: Dest::Bot(0),
                hi_dest: Dest::Bot(0),
                chips: Vec::new(),
            });
        }
        arena
    }

    fn reduce(&mut self) -> usize {
        self.bots.truncate(self.size);
        for ref v in self.inputs.iter() { 
            self.bots[v.destination].give(v.value);
        }
        loop {
            for i in 0..self.size {
                if !self.bots[i].ready() { continue; }
                if self.bots[i].chips.contains(&61) &&
                   self.bots[i].chips.contains(&17) {
                    return i;
                }
                if let Dest::Bot(lo) = self.bots[i].lo_dest {
                    let v = self.bots[i].chips[0];
                    self.bots[lo].give(v);
                }
                if let Dest::Bot(hi) = self.bots[i].hi_dest {
                    let v = self.bots[i].chips[1];
                    self.bots[hi].give(v);
                }
                self.bots[i].chips.clear();
            }
        }
    }
}

fn line_parse(mut arena: BotArena, line: String) -> BotArena {
    let toks: Vec<&str> = line.split_whitespace().collect();
    if toks[0]  == "value" {
        arena.inputs.push(Input {
            value: toks[1].parse::<usize>().unwrap(),
            destination: toks[5].parse::<usize>().unwrap(),
        });
    } else {
        let index = toks[1].parse::<usize>().unwrap();
        arena.size = cmp::max(index + 1, arena.size);
        arena.bots[index] = Bot {
            lo_dest: match toks[5] {
                "bot" => Dest::Bot(toks[6].parse::<usize>().unwrap()),
                _ => Dest::Out(toks[6].parse::<usize>().unwrap()),
            },
            hi_dest: match toks[10] {
                "bot" => Dest::Bot(toks[11].parse::<usize>().unwrap()),
                _ => Dest::Out(toks[11].parse::<usize>().unwrap()),
            },
            chips: Vec::new(),
        };
    }

    arena
}

fn main() {
    let reader = match File::open("input.txt") {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("Unable to open input.txt >:-( {}", e),
    };

    let arena = BotArena::new();
    let result = reader.lines()
        .fold(arena, |acc, l| {
            line_parse(acc, String::from(l.unwrap()))
        }).reduce();
    println!("Result: {}", result);
}
