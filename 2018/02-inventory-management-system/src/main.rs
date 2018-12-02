use std::cmp::min;

fn count(chars: &Vec<char>, occ: usize) -> usize {
    let mut set = chars.clone();
    let mut count = 0;
    set.dedup_by_key(|&mut i| i);
    for c1 in set.iter() {
        let mut ocs = 0;
        for c2 in chars.iter() {
            ocs += if c1 == c2 { 1 } else { 0 };
        }
        count += if ocs == occ { 1 } else { 0 };
    }

    min(count, 1)
}

fn solve_p1(input: &str) -> usize {
    let (p, t) = input.lines().fold((0, 0), |(p, t), l| {
        let mut chars = l.chars().collect::<Vec<char>>();
        chars.sort_unstable();
        (count(&chars, 2) + p, count(&chars, 3) + t)
    });
    p * t
}

fn count_diffs(s1: &str, s2: &str) -> usize {
    s1.chars()
        .zip(s2.chars())
        .map(|(c1, c2)| (c1 != c2) as usize)
        .sum()
}

fn solve_p2(input: &str) -> String {
    let entries = input.lines().collect::<Vec<&str>>();
    for (i, s1) in entries.iter().enumerate() {
        for (j, s2) in entries.iter().enumerate() {
            if i != j {
                if count_diffs(s1, s2) == 1 {
                    return s1
                        .chars()
                        .zip(s2.chars())
                        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
                        .collect::<String>();
                }
            }
        }
    }
    panic!();
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve_p1(input));
    println!("{}", solve_p2(input));
}

#[test]
fn test_simple() {
    let input = "\
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";
    assert_eq!(3 * 4, solve_p1(input));
}
