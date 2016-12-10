use std::io::{BufRead, BufReader};
use std::fs::File;

enum LedInstruction {
    Rect { width: usize, height: usize },
    RotateRow { row: usize, steps: usize },
    RotateCol { col: usize, steps: usize },
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
}
