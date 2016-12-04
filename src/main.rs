use std::cmp;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

struct Keypad {
    row: isize,
    col: isize,
}

enum Move {
    Up, Left, Down, Right,
}

fn clamp(num: isize) -> isize {
    cmp::min(2, cmp::max(num, 0))
}

impl Keypad {
    fn new() -> Keypad {
        Keypad {
            row: 1,
            col: 1,
        }
    }

    fn step(&mut self, mov: Move) {
        match mov {
            Move::Up => self.row -= 1,
            Move::Left => self.col -= 1,
            Move::Down => self.row += 1,
            Move::Right => self.col += 1,
        }
        self.row = clamp(self.row);
        self.col = clamp(self.col);
    }

    fn key(&self) -> isize {
        self.row * 3 + self.col + 1
    }
}

#[test]
fn simple_test() {
    let mut pad = Keypad::new();
    assert_eq!(pad.key(), 5);

    pad.step(Move::Up); 
    pad.step(Move::Left); 
    pad.step(Move::Left); 
    assert_eq!(pad.key(), 1);

    pad.step(Move::Right); 
    pad.step(Move::Right); 
    pad.step(Move::Down); 
    pad.step(Move::Down); 
    pad.step(Move::Down); 
    assert_eq!(pad.key(), 9);

    pad.step(Move::Left); 
    pad.step(Move::Up); 
    pad.step(Move::Right); 
    pad.step(Move::Down); 
    pad.step(Move::Left); 
    assert_eq!(pad.key(), 8);

    pad.step(Move::Up); 
    pad.step(Move::Up); 
    pad.step(Move::Up); 
    pad.step(Move::Up); 
    pad.step(Move::Down); 
    assert_eq!(pad.key(), 5);
}

fn main() {
    let reader = match File::open("input.txt") {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("Error opening file: {}", e),
    };
    
    let mut pad = Keypad::new();
    for line in reader.lines() {
        for ch in line.unwrap().chars() {
            pad.step(match ch {
                'U' => Move::Up,
                'L' => Move::Left,
                'D' => Move::Down,
                'R' => Move::Right,
                _ => panic!("Unexpected character {}", ch),
            });
        }
        println!("{}", pad.key());
    }
}
