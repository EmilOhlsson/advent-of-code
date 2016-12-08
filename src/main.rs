use std::io::{BufRead, BufReader};
use std::fs::File;

enum LedInstruction {
    Rect{w: usize, h: usize},
    RotateRow{row: usize, steps: usize},
    RotateCol{col: usize, steps: usize},
}

struct TinyLed {
}

impl TinyLed {
    fn new() -> TinyLed {
        TinyLed {} 
    }

    fn mutate(&mut self, instruction: LedInstruction) {
        unimplemented!();
    }

    fn pixelcount(&self) -> usize {
        unimplemented!();
    }
}

fn parse(inst: &str) -> Option<LedInstruction> {
    unimplemented!();
}

fn main() {
    let reader = match File::open("input.txt") {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("Unable to open file input.txt: {}"),
    };
    let mut display = TinyLed::new();
    for line in reader.lines() {
        display.mutate(parse(&line.unwrap()).unwrap());
    }
    println!("Pixels: {}", display.pixelcount());
}
