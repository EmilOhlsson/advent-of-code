use regex::Regex;
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Bot {
    pos: (i32, i32, i32),
    range: i32,
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

fn find_most(bot: &Bot, bots: &[Bot]) -> i32 {
    bots.iter()
        .map(|b| {
            let dx = b.pos.0 - bot.pos.0;
            let dy = b.pos.1 - bot.pos.1;
            let dz = b.pos.2 - bot.pos.2;
            if dx.abs() + dy.abs() + dz.abs() <= bot.range {
                1
            } else {
                0
            }
        })
        .sum()
}

fn solve_p1(input: &str) -> i32 {
    let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    let mut bots = input
        .lines()
        .map(|line| {
            let cpt = re.captures(line).unwrap();
            let vs = cpt
                .iter()
                .skip(1)
                .map(|s| s.unwrap().as_str().parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            Bot {
                pos: (vs[0], vs[1], vs[2]),
                range: vs[3],
            }
        })
        .collect::<Vec<Bot>>();
    bots.sort_unstable();
    find_most(&bots[0], &bots)
}

fn main() {
    let input = include_str!("input");
    println!("part 1: {}", solve_p1(input));
}

#[test]
fn test_p1() {
    let input = include_str!("input-test");
    assert_eq!(solve_p1(input), 7);
}
