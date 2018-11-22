extern crate regex;

use std::collections::VecDeque;

use regex::Regex;

struct Key {
    chs: VecDeque<char>,
}

impl Key {
    fn new(key: &str) -> Key {
        Key {
            chs: key.chars().collect::<VecDeque<char>>(),
        }
    }

    fn as_string(&self) -> String {
        self.chs.iter().collect::<String>()
    }

    fn swap_pos(&mut self, a: usize, b: usize) {
        self.chs.swap(a, b);
    }

    fn swap_letter(&mut self, a: char, b: char) {
        let p1 = self.chs.iter().position(|&p| p == a).unwrap();
        let p2 = self.chs.iter().position(|&p| p == b).unwrap();
        self.swap_pos(p1, p2);
    }

    fn rotate_steps(&mut self, dir: &str, a: usize) {
        if dir == "left" {
            for _ in 0..a {
                let ch = self.chs.pop_front().unwrap();
                self.chs.push_back(ch);
            }
        } else if dir == "right" {
            for _ in 0..a {
                let ch = self.chs.pop_back().unwrap();
                self.chs.push_front(ch);
            }
        }
    }

    fn rotate_pos(&mut self, letter: char) {
        let mut pos = self.chs.iter().position(|&p| p == letter).unwrap();
        if pos >= 4 {
            pos += 1;
        }
        self.rotate_steps("right", pos + 1);
    }

    fn reverse_pos(&mut self, mut a: usize, mut b: usize) {
        while a < b {
            self.chs.swap(a, b);
            a += 1;
            b -= 1;
        }
    }

    fn move_pos(&mut self, a: usize, b: usize) {
        let ch = self.chs.remove(a).unwrap();
        self.chs.insert(b, ch);
    }
}

fn solve_p1(key: &str, input: &str) -> String {
    let mut hash = Key::new(key);
    let swap_pos = Regex::new(r"swap position (\d) with position (\d)").unwrap();
    let swap_letter = Regex::new(r"swap letter (.) with letter (.)").unwrap();
    let rotate_steps = Regex::new(r"rotate (left|right) (\d) step").unwrap();
    let rotate_pos = Regex::new(r"rotate based on position of letter (.)").unwrap();
    let reverse_pos = Regex::new(r"reverse positions (\d) through (\d)").unwrap();
    let move_pos = Regex::new(r"move position (\d) to position (\d)").unwrap();

    for l in input.lines() {
        if let Some(cs) = swap_pos.captures(l) {
            hash.swap_pos(
                cs.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                cs.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            );
        } else if let Some(cs) = swap_letter.captures(l) {
            hash.swap_letter(
                cs.get(1).unwrap().as_str().parse::<char>().unwrap(),
                cs.get(2).unwrap().as_str().parse::<char>().unwrap(),
            );
        } else if let Some(cs) = rotate_steps.captures(l) {
            hash.rotate_steps(
                cs.get(1).unwrap().as_str(),
                cs.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            );
        } else if let Some(cs) = rotate_pos.captures(l) {
            hash.rotate_pos(cs.get(1).unwrap().as_str().parse::<char>().unwrap());
        } else if let Some(cs) = reverse_pos.captures(l) {
            hash.reverse_pos(
                cs.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                cs.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            );
        } else if let Some(cs) = move_pos.captures(l) {
            hash.move_pos(
                cs.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                cs.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            );
        } else {
            panic!("Unable to match: {}", l);
        }
    }

    hash.as_string()
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve_p1("abcdefgh", input));
}

#[test]
fn test() {
    assert_eq!(
        solve_p1("abcde", "swap position 4 with position 0"),
        "ebcda"
    );
    assert_eq!(solve_p1("ebcda", "swap letter d with letter b"), "edcba");
    assert_eq!(solve_p1("edcba", "reverse positions 0 through 4"), "abcde");
    assert_eq!(solve_p1("abcde", "rotate left 1 step"), "bcdea");
    assert_eq!(solve_p1("bcdea", "move position 1 to position 4"), "bdeac");
    assert_eq!(solve_p1("bdeac", "move position 3 to position 0"), "abdec");
    assert_eq!(
        solve_p1("abdec", "rotate based on position of letter b"),
        "ecabd"
    );
    assert_eq!(
        solve_p1("ecabd", "rotate based on position of letter d"),
        "decab"
    );
}
