use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Bot {
    pos: (i64, i64, i64),
    range: i64,
}

impl Bot {
    fn dist_from(&self, other: &Bot) -> i64 {
        let dx = self.pos.0 - other.pos.0;
        let dy = self.pos.1 - other.pos.1;
        let dz = self.pos.2 - other.pos.2;
        dx.abs() + dy.abs() + dz.abs()
    }

    fn rdist_p(&self, p: (i64, i64, i64)) -> i64 {
        let dx = self.pos.0 - p.0;
        let dy = self.pos.1 - p.1;
        let dz = self.pos.2 - p.2;
        dx.abs() + dy.abs() + dz.abs() - self.range
    }
}

impl Ord for Bot {
    fn cmp(&self, other: &Bot) -> Ordering {
        other.range.cmp(&self.range)
    }
}

impl PartialOrd for Bot {
    fn partial_cmp(&self, other: &Bot) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_most(bot: &Bot, bots: &[Bot]) -> i64 {
    bots.iter()
        .map(|b| (bot.dist_from(b) <= bot.range) as i64)
        .sum()
}

fn build_bots(input: &str) -> Vec<Bot> {
    let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let cpt = re.captures(line).unwrap();
            let vs = cpt
                .iter()
                .skip(1)
                .map(|s| s.unwrap().as_str().parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            Bot {
                pos: (vs[0], vs[1], vs[2]),
                range: vs[3],
            }
        })
        .collect::<Vec<Bot>>()
}

fn solve_p1(input: &str) -> i64 {
    let mut bots = build_bots(input);
    bots.sort_unstable();
    find_most(&bots[0], &bots)
}

fn solve_p2(input: &str) -> i64 {
    let bots = build_bots(input);
    let mut xs = bots.iter().map(|b| b.pos.0).minmax().into_option().unwrap();
    let mut ys = bots.iter().map(|b| b.pos.1).minmax().into_option().unwrap();
    let mut zs = bots.iter().map(|b| b.pos.2).minmax().into_option().unwrap();

    let mut range = 1;
    while range < xs.1 - xs.0 {
        range <<= 1;
    }

    loop {
        let mut count = 0;
        let mut candidate = (0, 0, 0);
        let mut value = 0;

        for x in (xs.0..=xs.1).step_by(range as usize) {
            for y in (ys.0..=ys.1).step_by(range as usize) {
                for z in (zs.0..=zs.1).step_by(range as usize) {
                    let c = bots
                        .iter()
                        .filter(|b| b.rdist_p((x, y, z)) / range <= 0)
                        .count();
                    if c > count {
                        count = c;
                        candidate = (x, y, z);
                        value = x.abs() + y.abs() + z.abs();
                    }
                }
            }
        }

        if range == 1 {
            return value;
        }

        xs = (candidate.0 - range, candidate.0 + range);
        ys = (candidate.1 - range, candidate.1 + range);
        zs = (candidate.2 - range, candidate.2 + range);

        range >>= 1;
    }
}

fn main() {
    let input = include_str!("input");
    println!("part 1: {}", solve_p1(input));
    println!("part 2: {}", solve_p2(input));
}

#[test]
fn test_p1() {
    let input = include_str!("input-test");
    assert_eq!(solve_p1(input), 7);
}

#[test]
fn test_p2() {
    let input = include_str!("input-test-2");
    assert_eq!(solve_p2(input), 36);
}
