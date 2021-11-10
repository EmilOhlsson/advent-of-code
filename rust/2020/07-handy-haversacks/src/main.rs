use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

fn solve(input: &str) -> (u32, u32) {
    let mut parent_map = HashMap::<String, Vec<String>>::new();
    let mut content_map = HashMap::<String, Vec<(u32, String)>>::new();
    let line_re = Regex::new(r"^(\w+ \w+) bags contain (.*).$").unwrap();
    let content_re = Regex::new(r"(\d+) (\w+ \w+)").unwrap();
    for line in input.lines() {
        if let Some(bag_cap) = line_re.captures(line) {
            for content_cap in content_re.captures_iter(&bag_cap[2]) {
                let count = content_cap[1].parse::<u32>().unwrap();
                // Parent mapping
                {
                    let entry = parent_map
                        .entry(content_cap[2].to_string())
                        .or_insert_with(Vec::new);
                    entry.push(bag_cap[1].to_string());
                }
                // Content mapping
                {
                    let entry = content_map
                        .entry(bag_cap[1].to_string())
                        .or_insert_with(Vec::new);
                    entry.push((count, content_cap[2].to_string()));
                }
            }
        } else {
            panic!("Unable to parse line: {}", line);
        }
    }

    let mut count_p1 = 0;
    let mut count_p2 = 0;
    // Part 1
    {
        let mut queue = VecDeque::<String>::new();
        let mut enqueued = HashSet::<String>::new();
        queue.push_back("shiny gold".to_string());
        while let Some(bag) = queue.pop_front() {
            if let Some(containers) = parent_map.get(&bag) {
                for b in containers {
                    if enqueued.insert(b.to_string()) {
                        queue.push_back(b.to_string());
                        count_p1 += 1;
                    }
                }
            }
        }
    }

    // Part 2
    {
        let mut queue = VecDeque::<(u32, String)>::new();
        queue.push_back((1, "shiny gold".to_string()));
        while let Some((cnt, bag)) = queue.pop_front() {
            if let Some(content) = content_map.get(&bag) {
                for (c, b) in content {
                    queue.push_back(((cnt * c), b.to_string()));
                    count_p2 += cnt * c;
                }
            }
        }
    }
    (count_p1, count_p2)
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input));
}

#[test]
fn test() {
    let input = include_str!("input-simple");
    assert_eq!(solve(input), (4, 32));
}
