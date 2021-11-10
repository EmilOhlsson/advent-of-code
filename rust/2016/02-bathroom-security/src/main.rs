use std::cmp;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

struct Keypad {
    row: usize,
    col: usize,
}

static KPAD: [[char; 5]; 5] = [
    [' ', ' ', '1', ' ', ' '],
    [' ', '2', '3', '4', ' '],
    ['5', '6', '7', '8', '9'],
    [' ', 'A', 'B', 'C', ' '],
    [' ', ' ', 'D', ' ', ' '],
];

enum Move {
    Up, Left, Down, Right,
}

fn clamp(num: isize) -> usize {
    cmp::min(KPAD.len() - 1, cmp::max(num, 0) as usize)
}

impl Keypad {
    fn new() -> Keypad {
        Keypad {
            row: 2,
            col: 2,
        }
    }

    fn step(&mut self, mov: Move) {
        let (r, c)  = match mov {
            Move::Up => (clamp(self.row as isize - 1), self.col),
            Move::Left => (self.row, clamp(self.col as isize - 1)),
            Move::Down => (clamp(self.row as isize + 1), self.col),
            Move::Right => (self.row, clamp(self.col as isize + 1)),
        };
        
        if KPAD[r][c].is_alphanumeric() {
            self.row = r;
            self.col = c;
        }
    }

    fn key(&self) -> char {
        KPAD[self.row][self.col]
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
