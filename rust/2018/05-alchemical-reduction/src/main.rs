use std::collections::VecDeque;

fn polarity_match(a: char, b: char) -> bool {
    (a.is_uppercase() != b.is_uppercase()) && (a.to_ascii_lowercase() == b.to_ascii_lowercase())
}

fn solve(input: &str) -> usize {
    let mut new_chars: VecDeque<char> = VecDeque::new();

    for c in input.chars() {
        assert!(c.is_alphabetic()); /* Every single time.. */
        if let Some(prev) = new_chars.pop_back() {
            if !polarity_match(prev, c) {
                new_chars.push_back(prev);
                new_chars.push_back(c);
            }
        } else {
            new_chars.push_back(c);
        }
    }

    new_chars.len()
}

fn solve_p2(input: &str) -> usize {
    "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|c| {
            input
                .chars()
                .filter(|l| l.to_ascii_lowercase() != c)
                .collect::<String>()
        }).map(|s| solve(&s))
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("input.txt").trim();
    let solution = solve(input);
    println!("{}", solution);
    println!("{}", solve_p2(input));
}

#[test]
fn test_polarity() {
    assert!(polarity_match('a', 'A'));
    assert!(polarity_match('B', 'b'));
    assert!(polarity_match('c', 'C'));
    assert!(!polarity_match('A', 'A'));
    assert!(!polarity_match('b', 'b'));
    assert!(!polarity_match('a', 'B'));
    assert!(!polarity_match('A', 'b'));
    assert!(!polarity_match('d', 'a'));
    assert!(!polarity_match('c', 'a'));
}

#[test]
fn test() {
    assert_eq!(solve("dabAcCaCBAcCcaDA"), 10);
}
