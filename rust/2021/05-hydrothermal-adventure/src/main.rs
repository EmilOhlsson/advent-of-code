use regex::Regex;
use std::cmp;
use std::collections::HashMap;

struct Range {
    start: i32,
    step: i32,
    steps: i32,
}

impl Range {
    fn new(start: i32, end: i32, steps: i32) -> Range {
        Range {
            start,
            step: (end - start).clamp(-1, 1),
            steps,
        }
    }
}

impl Iterator for Range {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.steps == 0 {
            None
        } else {
            let result = Some(self.start);
            self.start += self.step;
            self.steps -= 1;
            result
        }
    }
}

fn parse(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            let x1 = cap[1].parse::<i32>().unwrap();
            let y1 = cap[2].parse::<i32>().unwrap();
            let x2 = cap[3].parse::<i32>().unwrap();
            let y2 = cap[4].parse::<i32>().unwrap();
            ((x1, y1), (x2, y2))
        })
        .collect()
}

fn solve_p1(input: &str) -> usize {
    let mut lines = HashMap::<(i32, i32), u32>::new();
    for ((x1, y1), (x2, y2)) in parse(input) {
        if x1 == x2 || y1 == y2 {
            let steps = cmp::max((x2 - x1).abs(), (y2 - y1).abs()) + 1;
            let xs = Range::new(x1, x2, steps);
            let ys = Range::new(y1, y2, steps);
            for p in xs.zip(ys) {
                let point = lines.entry(p).or_insert(0);
                *point += 1;
            }
        }
    }
    lines.values().filter(|&c| *c >= 2).count()
}

fn solve_p2(input: &str) -> usize {
    let mut lines = HashMap::<(i32, i32), u32>::new();
    for ((x1, y1), (x2, y2)) in parse(input) {
        let steps = cmp::max((x2 - x1).abs(), (y2 - y1).abs()) + 1;
        let xs = Range::new(x1, x2, steps);
        let ys = Range::new(y1, y2, steps);
        for p in xs.zip(ys) {
            let point = lines.entry(p).or_insert(0);
            *point += 1;
        }
    }
    lines.values().filter(|&c| *c >= 2).count()
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve_p1(input));
    println!("{}", solve_p2(input));
}

#[test]
fn test_simple() {
    let input = include_str!("input-simple");
    assert_eq!(solve_p1(input), 5);
    assert_eq!(solve_p2(input), 12);
}
