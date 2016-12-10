use std::io::{BufRead, BufReader};
use std::fs::File;
use std::fmt;

enum LedInstruction {
    Rect { width: usize, height: usize },
    RotateRow { row: usize, steps: usize },
    RotateCol { col: usize, steps: usize },
}

struct TinyLed {
    leds: [[bool; 6]; 50],
}

impl TinyLed {
    fn new() -> TinyLed {
        TinyLed { leds: [[false; 6]; 50] }
    }

    fn fmt(&self) {
        for r in 0..6 {
            for c in 0..50 {
                if self.leds[c][r] {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }

    fn shiftcol(&mut self, col: usize) {
        let mut prev_val: bool = self.leds[col][5];
        let mut prev_val_tmp: bool;
        for i in 0..6 {
            prev_val_tmp = self.leds[col][i];
            self.leds[col][i] = prev_val;
            prev_val = prev_val_tmp;
        }
    }

    fn shiftrow(&mut self, row: usize) {
        let mut prev_val: bool = self.leds[49][row];
        let mut prev_val_tmp: bool;
        for i in 0..50 {
            prev_val_tmp = self.leds[i][row];
            self.leds[i][row] = prev_val;
            prev_val = prev_val_tmp;
        }
    }

    fn mutate(&mut self, instruction: LedInstruction) {
        match instruction {
            LedInstruction::Rect { width, height } => {
                for h in 0..height {
                    for w in 0..width {
                        self.leds[w][h] = true;
                    }
                }
            },
            LedInstruction::RotateRow { row, steps } => {
                for _ in 0..steps {
                    self.shiftrow(row);
                }
            },
            LedInstruction::RotateCol { col, steps } => {
                for _ in 0..steps {
                    self.shiftcol(col);
                }
            },
        }
    }

    fn pixelcount(&self) -> usize {
        self.leds
            .iter()
            .map(|col| col.iter().fold(0, |acc: usize, &x| if x { acc + 1 } else { acc }))
            .sum()
    }
}

#[test]
fn test_mutate() {
    let mut disp = TinyLed::new();
    disp.mutate(LedInstruction::Rect {
        width: 3,
        height: 2,
    });
    assert_eq!(disp.pixelcount(), 6);
}

#[test]
fn test_mutate_line() {
    let mut disp = TinyLed::new();
    disp.mutate(LedInstruction::Rect {
        width: 25,
        height: 1,
    });
    assert_eq!(disp.pixelcount(), 25);
}

fn parse(inst: &str) -> LedInstruction {
    let inst_str = String::from(inst);
    let toks: Vec<&str> = inst_str.split_whitespace().collect();
    match toks[0] {
        "rect" => {
            let rect_str = String::from(toks[1]);
            let rect_toks: Vec<&str> = rect_str.split('x').collect();
            LedInstruction::Rect {
                width: rect_toks[0].parse::<usize>().unwrap(),
                height: rect_toks[1].parse::<usize>().unwrap(),
            }
        }
        "rotate" => {
            match toks[1] {
                "row" => {
                    let row_str = String::from(toks[2]);
                    let row_tok: Vec<&str> = row_str.split('=').collect();
                    LedInstruction::RotateRow {
                        row: row_tok[1].parse::<usize>().unwrap(),
                        steps: toks[4].parse::<usize>().unwrap(),
                    }
                }
                "column" => {
                    let col_str = String::from(toks[2]);
                    let col_tok: Vec<&str> = col_str.split('=').collect();
                    LedInstruction::RotateCol {
                        col: col_tok[1].parse::<usize>().unwrap(),
                        steps: toks[4].parse::<usize>().unwrap(),
                    }
                }
                _ => panic!("Unknown dimension: {}", toks[1]),
            }
        }
        _ => panic!("Invalid keyword: {}", toks[0]),
    }
}

#[test]
fn test_parse() {
    let rect = parse("rect 3x2");
    let rot_col = parse("rotate column x=1 by 1");
    let rot_row = parse("rotate row y=1 by 1");
    assert_eq!(match rect {
                   LedInstruction::Rect { width: w, height: h } => (w, h),
                   _ => panic!(),
               },
               (3, 2));
    assert_eq!(match rot_row {
                   LedInstruction::RotateRow { row: r, steps: s } => (r, s),
                   _ => panic!(),
               },
               (1, 1));
    assert_eq!(match rot_col {
                   LedInstruction::RotateCol { col: c, steps: s } => (c, s),
                   _ => panic!(),
               },
               (1, 1));
}

fn main() {
    let reader = match File::open("input.txt") {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("Unable to open file input.txt: {}", e),
    };
    let mut display = TinyLed::new();
    for line in reader.lines() {
        display.mutate(parse(&line.unwrap()));
    }
    println!("Pixels: {}", display.pixelcount());

    display.fmt();
}
