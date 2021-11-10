use regex::{Match, Regex};
use std::collections::{HashMap, HashSet};

type Relations = HashMap<String, HashMap<String, i32>>;

fn parse(input: &str) -> HashMap<String, HashMap<String, i32>> {
    let mut map = HashMap::<String, HashMap<String, i32>>::new();
    let as_str = |v: Option<Match>| v.unwrap().as_str().to_string();
    let as_num = |v: Option<Match>| v.unwrap().as_str().parse::<i32>().unwrap();
    let re = Regex::new(r"(?P<name>\p{Alphabetic}+) would (?P<isFriend>gain|lose) (?P<amount>\d+) happiness units by sitting next to (?P<friend>\p{Alphabetic}+)").unwrap();
    for cap in input.lines().map(|line| re.captures(line).unwrap()) {
        let change = if as_str(cap.name("isFriend")) == "gain" {
            as_num(cap.name("amount"))
        } else {
            -as_num(cap.name("amount"))
        };
        let rel = map
            .entry(as_str(cap.name("name")))
            .or_insert_with(HashMap::new);
        rel.insert(as_str(cap.name("friend")), change);
    }

    map
}

fn eval_seating(seating: &Vec<String>, rel: &Relations) -> i32 {
    let len = seating.len() as i32;
    let index = |v: i32| if v < 0 { v + len } else if v >= len { v - len } else {v} as usize;
    seating
        .iter()
        .enumerate()
        .map(|(i, g)| {
            let left = rel
                .get(g)
                .map(|n| n.get(&seating[index(i as i32 - 1)]).unwrap_or(&0))
                .unwrap_or(&0);
            let right = rel
                .get(g)
                .map(|n| n.get(&seating[index(i as i32 + 1)]).unwrap_or(&0))
                .unwrap_or(&0);
            left + right
        })
        .sum()
}

fn seat(
    rel: &Relations,
    seating: &Vec<String>,
    seated: &HashSet<String>,
    guests: &HashSet<String>,
) -> i32 {
    if seated.len() == guests.len() {
        eval_seating(&seating, rel)
    } else {
        guests
            .difference(&seated)
            .map(|guest| {
                let mut seating_new = seating.clone();
                let mut seated_new = seated.clone();
                seating_new.push(guest.to_string());
                seated_new.insert(guest.to_string());
                seat(rel, &seating_new, &seated_new, guests)
            })
            .max()
            .unwrap()
    }
}

fn solve(input: &str) -> i32 {
    let rel = parse(input);
    let guests = rel.keys().cloned().collect::<HashSet<String>>();
    seat(&rel, &Vec::new(), &HashSet::new(), &guests)
}

fn solve_v2(input: &str) -> i32 {
    let rel = parse(input);
    let mut guests = rel.keys().cloned().collect::<HashSet<String>>();
    guests.insert("you".to_string());
    seat(&rel, &Vec::new(), &HashSet::new(), &guests)
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input));
    println!("{}", solve_v2(input));
}

#[test]
fn test() {
    let input = include_str!("input-simple");
    assert_eq!(solve(input), 330);
}
