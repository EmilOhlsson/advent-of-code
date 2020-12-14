use regex::Regex;
use std::collections::HashMap;

fn solve_p1(input: &str) -> u64 {
    let mut memory = HashMap::<usize, u64>::new();
    let re_mask = Regex::new(r"^mask = ([X10]{36})$").unwrap();
    let re_write = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let mut ones_mask = 0u64;
    let mut zero_mask = 0u64;

    for line in input.lines() {
        if let Some(cap) = re_mask.captures(line) {
            ones_mask = cap[1]
                .chars()
                .rev()
                .enumerate()
                .filter_map(|(i, ch)| {
                    if ch == '1' {
                        Some(1u64 << i as u64)
                    } else {
                        None
                    }
                })
                .fold(0, |acc, v| acc | v);
            zero_mask = cap[1]
                .chars()
                .rev()
                .enumerate()
                .filter_map(|(i, ch)| {
                    if ch != '0' {
                        Some(1u64 << i as u64)
                    } else {
                        None
                    }
                })
                .fold(0, |acc, v| acc | v);
        } else if let Some(cap) = re_write.captures(line) {
            let index = cap[1].parse::<usize>().unwrap();
            let value = cap[2].parse::<u64>().unwrap();
            memory.insert(index, (value & zero_mask) | ones_mask);
        } else {
            panic!("Failed on: {}", line);
        }
    }

    memory.values().sum::<u64>()
}

fn solve_p2(input: &str) -> u64 {
    let mut memory = HashMap::<u64, u64>::new();
    let re_mask = Regex::new(r"^mask = ([X10]{36})$").unwrap();
    let re_write = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let mut ones_mask = 0u64;
    let mut zero_mask = 0u64;
    let mut flot_mask = 0u64;

    for line in input.lines() {
        if let Some(cap) = re_mask.captures(line) {
            ones_mask = 0;
            zero_mask = 0;
            flot_mask = 0;
            for (i, ch) in cap[1].chars().rev().enumerate() {
                match ch {
                    '0' => (),
                    '1' => {
                        ones_mask |= 1 << i as u64;
                        zero_mask |= 1 << i as u64;
                    }
                    'X' => {
                        flot_mask |= 1 << i as u64;
                        zero_mask |= 1 << i as u64;
                    }
                    _ => panic!("Did not recognize '{}", ch),
                }
            }
        } else if let Some(cap) = re_write.captures(line) {
            let index = cap[1].parse::<u64>().unwrap();
            let value = cap[2].parse::<u64>().unwrap();

            // The bits that are floating for this address
            let floating = (flot_mask ^ (flot_mask & ones_mask)) & zero_mask;
            let base_address = (index | ones_mask) & !floating;

            let nvalues = floating.count_ones();
            let count_mask = (1 << nvalues) - 1;
            for v in 0..=count_mask {
                // Yuck, much room for improvement
                let mut n = 0;
                let mut current = base_address;
                for i in 0..36 {
                    if (1 << i) & floating != 0 {
                        current |= ((v >> n) & 1) << i;
                        n += 1;
                    }
                }
                memory.insert(current, value);
            }
        } else {
            panic!("Failed on: {}", line);
        }
    }

    memory.values().sum::<u64>()
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve_p1(input));
    println!("{}", solve_p2(input));
}

#[test]
fn test_simple() {
    let input = include_str!("input-simple");
    assert_eq!(solve_p1(input), 165);
}

#[test]
fn test_simple_p2() {
    let input = include_str!("input-simple-p2");
    assert_eq!(solve_p2(input), 208);
}
