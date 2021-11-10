use std::collections::{HashSet, VecDeque};

/// Build two decks from the input
fn parse(input: &str) -> [VecDeque<usize>; 2] {
    let mut decks = [VecDeque::<usize>::new(), VecDeque::<usize>::new()];
    let mut i = 0;
    for line in input.lines() {
        if line.is_empty() {
            i += 1;
        } else if !line.starts_with("Player") {
            decks[i].push_back(line.parse::<usize>().unwrap());
        }
    }
    decks
}

/// Calculates the score of a deck
fn score(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, d)| (i + 1) * d)
        .sum()
}

fn solve_v1(input: &str) -> usize {
    let mut decks = parse(input);

    // Play until one deck is empty
    while !decks[0].is_empty() && !decks[1].is_empty() {
        let a = decks[0].pop_front().unwrap();
        let b = decks[1].pop_front().unwrap();
        if a > b {
            decks[0].push_back(a);
            decks[0].push_back(b);
        } else {
            decks[1].push_back(b);
            decks[1].push_back(a);
        }
    }

    std::cmp::max(score(&decks[0]), score(&decks[1]))
}

fn recursive_comabat(decks: &mut [VecDeque<usize>; 2]) {
    let mut states = HashSet::new();
    while !decks[0].is_empty() && !decks[1].is_empty() {
        if states.insert(decks.clone()) {
            let a = decks[0].pop_front().unwrap();
            let b = decks[1].pop_front().unwrap();
            let player0_won = if a <= decks[0].len() && b <= decks[1].len() {
                let mut decks_copy = [
                    decks[0].iter().take(a).cloned().collect(),
                    decks[1].iter().take(b).cloned().collect(),
                ];
                recursive_comabat(&mut decks_copy);
                decks_copy[1].is_empty()
            } else {
                a > b
            };
            if player0_won {
                decks[0].push_back(a);
                decks[0].push_back(b);
            } else {
                decks[1].push_back(b);
                decks[1].push_back(a);
            }
        } else {
            decks[1].clear();
        }
    }
}

fn solve_v2(input: &str) -> usize {
    let mut decks = parse(input);

    recursive_comabat(&mut decks);

    std::cmp::max(score(&decks[0]), score(&decks[1]))
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve_v1(input));
    println!("{}", solve_v2(input));
}

#[test]
fn test_p1() {
    let input = include_str!("input-simple");
    assert_eq!(solve_v1(input), 306);
}

#[test]
fn test_p2() {
    let input = include_str!("input-simple");
    assert_eq!(solve_v2(input), 291);
}
