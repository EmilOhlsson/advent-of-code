use regex::Regex;
use std::collections::{HashMap, HashSet};

fn solve(input: &str) -> String {
    let re = Regex::new(r"Step (.) must be finished before step (.) can begin.").unwrap();
    let graph: HashMap<char, Vec<char>> = input
        .lines()
        .map(|l| {
            let capt = re.captures(l).unwrap();
            let a = capt[1].parse::<char>().unwrap();
            let b = capt[2].parse::<char>().unwrap();
            (a, b)
        })
        .fold(HashMap::new(), |mut map, (a, b)| {
            let e = map.entry(b).or_insert(Vec::new());
            e.push(a);
            map.entry(a).or_insert(Vec::new());
            map
        });

    let mut order = String::new();
    let mut done: HashSet<char> = HashSet::new();
    while order.len() < graph.len() {
        let mut avail = graph
            .iter()
            .filter_map(|(a, n)| {
                let pending = n
                    .iter()
                    .filter(|n| !done.contains(n))
                    .count();
                if done.contains(a) || pending > 0 {
                    None
                } else {
                    Some(*a)
                }
            })
            .collect::<Vec<char>>();
        avail.sort_unstable();
        done.insert(avail[0]);
        order.push(avail[0]);
    }
    order
}

fn main() {
    let input = include_str!("input");
    let solution = solve(input.trim());
    println!("{}", solution);
    // SLTKHXGRJAOMVDNQZCBWPIYFUE is wrong
    // JKNSTRHGCBVDXWAYOQPMLFZUIE is wrong (but code passes locally)
    // JKNSTRHCVDXWABYFGOQLMZPUIE is wrong
    // JKNSTHCBGRVDXWAYFOQLMPZIUE is better
}

#[test]
fn test() {
    let simple = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
    assert_eq!(solve(simple), "CABDFE");
}
