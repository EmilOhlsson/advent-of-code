use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;

enum Dest {
    Node(usize),
    Out(usize),
}

struct Input {
    value: usize,
    destination: Dest,
}

struct Bot {
    lo_dest: Dest,
    hi_dest: Dest,
    inputs: Vec<usize>,
}

struct BotArena {
    bots: Vec<Bot>,
    inputs: Vec<Input>,
}

impl BotArena {
    fn new() -> BotArena {
        BotArena {
            bots: Vec::with_capacity(256),
            inputs: Vec::new(),
        }
    }

    fn reduce(&mut self) -> usize {
        0
    }
}

fn line_parse(mut arena: BotArena, line: String) -> BotArena {
    let toks: Vec<&str> = line.split_whitespace().collect();
    if toks[0]  == "value" {
        arena.inputs.push(Input {
            value: toks[1].parse::<usize>().unwrap(),
            destination: Dest::Node(toks[5].parse::<usize>().unwrap()),
        });
    } else {
        arena.bots[toks[1].parse::<usize>().unwrap()] = Bot {
            lo_dest: match toks[5] {
                "bot" => Dest::Node(toks[6].parse::<usize>().unwrap()),
                _ => Dest::Out(toks[6].parse::<usize>().unwrap()),
            },
            hi_dest: match toks[10] {
                "bot" => Dest::Node(toks[11].parse::<usize>().unwrap()),
                _ => Dest::Out(toks[11].parse::<usize>().unwrap()),
            },
            inputs: Vec::new(),
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
