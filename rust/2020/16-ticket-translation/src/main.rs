use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
struct Range {
    lo: u32,
    hi: u32,
}

impl Range {
    fn new((lo, hi): (u32, u32)) -> Range {
        Range { lo, hi }
    }

    fn matches(&self, value: u32) -> bool {
        self.lo <= value && value <= self.hi
    }
}

fn solve(input: &str) -> (u32, u64) {
    let rule_re = Regex::new(r"^([a-z\s]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let ticket_re = Regex::new(r"^\d+(,\d+)*$").unwrap();

    let mut rules = HashMap::<String, (Range, Range)>::new();
    let mut tickets = Vec::<Vec<u32>>::new();
    let mut discards = HashSet::<usize>::new();

    for line in input.lines() {
        if let Some(rule) = rule_re.captures(line) {
            let range_lo = (
                rule[2].parse::<u32>().unwrap(),
                rule[3].parse::<u32>().unwrap(),
            );
            let range_hi = (
                rule[4].parse::<u32>().unwrap(),
                rule[5].parse::<u32>().unwrap(),
            );
            rules.insert(
                rule[1].to_string(),
                (Range::new(range_lo), Range::new(range_hi)),
            );
        } else if let Some(ticket) = ticket_re.captures(line) {
            let ticket_nums = ticket[0]
                .split(',')
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .collect::<Vec<u32>>();
            tickets.push(ticket_nums);
        }
    }

    let mut err_sum = 0;
    for (id, ticket) in tickets.iter().enumerate() {
        for field in ticket {
            let mut valid = false;
            for (range_lo, range_hi) in rules.values() {
                valid |= range_lo.matches(*field);
                valid |= range_hi.matches(*field);
                if valid {
                    break;
                }
            }
            if !valid {
                err_sum += *field;
                discards.insert(id);
            }
        }
    }

    // Create array of hashmaps (candidates per column)
    let mut candidates = Vec::new();
    for _ in 0..tickets[0].len() {
        candidates.push(rules.clone());
    }

    // Run through all valid tickets, and eliminate candidates
    for (_, ticket) in tickets
        .iter()
        .enumerate()
        .filter(|(id, _)| !discards.contains(&id))
    {
        for (col, value) in ticket.iter().enumerate() {
            let mut tmp = HashMap::new();
            for (rule, (r0, r1)) in candidates[col].drain() {
                if r0.matches(*value) || r1.matches(*value) {
                    tmp.insert(rule, (r0, r1));
                }
            }
            candidates[col] = tmp;
        }
    }

    // Reduce candidates by identifying specific rules. Run until all colums has
    // specific rules
    let mut done = false;
    while !done {
        let specific = candidates
            .iter()
            .filter_map(|c| if c.len() == 1 { c.keys().next() } else { None })
            .cloned()
            .collect::<Vec<String>>();
        for candidate in candidates.iter_mut() {
            if candidate.len() > 1 {
                for spec in &specific {
                    candidate.remove(spec);
                }
            }
        }
        done = !candidates.iter().any(|c| c.len() > 1);
    }

    // Find values for part 2
    let mut product = 1;
    for (col, rule) in candidates.iter().enumerate() {
        if rule.keys().next().unwrap().starts_with("departure") {
            product *= tickets[0][col] as u64;
        }
    }

    (err_sum, product)
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input));
}

#[test]
fn test_p1() {
    let input = include_str!("input-simple");
    assert_eq!(solve(input).0, 71);
}
