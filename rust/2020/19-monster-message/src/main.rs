use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Rule {
    Alternative(Vec<usize>, Vec<usize>),
    Sym(char),
    Pattern(Vec<usize>),
}

fn to_nums(nums: &str) -> Vec<usize> {
    nums.split_whitespace()
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect()
}

fn as_regex(idx: usize, rules: &HashMap<usize, Rule>, part2: bool) -> String {
    if part2 && idx == 8 {
        format!("{}+", as_regex(42, rules, part2))
    } else if part2 && idx == 11 {
        // uhg...
        let p1 = as_regex(42, rules, part2);
        let p2 = as_regex(31, rules, part2);
        let re = (1..=8)
            .map(|n| format!("{}{{{}}}{}{{{}}}", p1, n, p2, n))
            .collect::<Vec<String>>()
            .join("|");
        format!("({})", re)
    } else {
        match &rules[&idx] {
            Rule::Alternative(pat0, pat1) => format!(
                "({}|{})",
                pat0.iter()
                    .map(|i| as_regex(*i, rules, part2))
                    .collect::<Vec<String>>()
                    .join(""),
                pat1.iter()
                    .map(|i| as_regex(*i, rules, part2))
                    .collect::<Vec<String>>()
                    .join("")
            ),
            Rule::Sym(ch) => String::from(*ch),
            Rule::Pattern(pat) => format!(
                "({})",
                pat.iter()
                    .map(|i| as_regex(*i, rules, part2))
                    .collect::<Vec<String>>()
                    .join("")
            ),
        }
    }
}

fn solve(input: &str, part2: bool) -> u32 {
    let re_alt = Regex::new(r"^(\d+( \d+)*) \| (\d+( \d+)*)$").unwrap();
    let re_pattern = Regex::new(r"^(\d+( \d+)*)$").unwrap();
    let re_sym = Regex::new("^\"(\\w)\"$").unwrap();
    let mut check_message = false;
    let mut rules = HashMap::<usize, Rule>::new();
    let mut regex: Option<Regex> = None;

    let mut count = 0;
    for line in input.lines() {
        if line == "" {
            check_message = true;
            let regex_str = format!("^{}$", as_regex(0, &rules, part2));
            regex = Some(Regex::new(&regex_str).unwrap());
            continue;
        }
        if check_message {
            count += regex.as_ref().unwrap().is_match(line) as u32;
        } else {
            let mut tok_itr = line.split(':').map(str::trim);
            let i = tok_itr.next().unwrap().parse::<usize>().unwrap();
            let rule_text = tok_itr.next().unwrap();
            rules.insert(
                i,
                if let Some(cap) = re_alt.captures(rule_text) {
                    Rule::Alternative(to_nums(&cap[1]), to_nums(&cap[3]))
                } else if let Some(cap) = re_sym.captures(rule_text) {
                    Rule::Sym(cap[1].chars().next().unwrap())
                } else if let Some(cap) = re_pattern.captures(rule_text) {
                    Rule::Pattern(to_nums(&cap[1]))
                } else {
                    panic!("was not able to build rule from: {}", rule_text)
                },
            );
        }
    }
    count
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input, false));
    println!("{}", solve(input, true));
}

#[test]
fn test_simple_0() {
    let input = include_str!("input-simple-0");
    assert_eq!(solve(input, false), 2);
}

#[test]
fn test_simple_1() {
    let input = include_str!("input-simple-1");
    assert_eq!(solve(input, false), 2);
}
