use regex::Regex;
use std::collections::{HashMap, HashSet};

fn solve(input: &str, workers: usize) -> (String, usize) {
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
    let mut in_progress: HashSet<char> = HashSet::new();
    let mut time = 0;
    let mut work: Vec<(usize, Option<char>)> = vec![(0, None); workers];
    while done.len() < graph.len() {
        // Check for finished work
        for w in work.iter_mut() {
            if w.0 > 0 {
                w.0 -= 1;
                if w.0 == 0 {
                    done.insert(w.1.unwrap());
                    w.1 = None;
                }
            }
        }
        let mut avail = graph
            .iter()
            .filter_map(|(a, n)| {
                let pending = n.iter().filter(|n| !done.contains(n)).count();
                if done.contains(a) || pending > 0 {
                    None
                } else {
                    Some(*a)
                }
            })
            .collect::<Vec<char>>();

        avail.sort_unstable();
        for i in 0..std::cmp::min(avail.len(), workers) {
            if !in_progress.contains(&avail[i]) {
                for w in work.iter_mut() {
                    if w.0 == 0 {
                        let t = avail[i] as usize - 4;
                        //print!("{}: {} ({} s)  ", i, avail[i], t);
                        w.0 = avail[i] as usize - 4;
                        w.1 = Some(avail[i]);
                        order.push(avail[i]);
                        in_progress.insert(avail[i]);
                        break;
                    }
                }
            }
        }
        //println!("");
        println!(
            "{}: avail: {:?}. {} free workers -- {:?}",
            time,
            avail,
            work.iter().filter(|&w| w.0 == 0).count(),
            work
        );
        time += 1;
    }
    (order, time - 1)
}

fn main() {
    let input = include_str!("input");
    let solution = solve(input.trim(), 4);
    println!("{}, {}", solution.0, solution.1);
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
    assert_eq!(solve(simple, 2), ("CABDFE".to_owned(), 15));
}
