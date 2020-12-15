#[derive(Debug, Default, Clone, Copy)]
struct Num {
    turn: u32,
    age: u32,
    count: u32,
}

impl Num {
    fn new(turn: u32, age: u32, count: u32) -> Num {
        Num { turn, age, count }
    }
}

fn solve(nums: &[u32], turns: u32) -> u32 {
    let mut spoken = std::collections::HashMap::<u32, Num>::new();
    let mut prev_num = 0;
    let mut prev_entry = Default::default();
    for (i, n) in nums.iter().enumerate() {
        prev_entry = Num::new(i as u32 + 1, 0, 1);
        prev_num = *n;
        spoken.insert(*n, prev_entry);

    }

    for turn in nums.len() as u32 + 1..=turns {
        let num = prev_entry;
        if num.count == 1 {
            prev_num = 0;
        } else {
            prev_num = num.age;
        }
        let entry = spoken.entry(prev_num).or_insert_with(Default::default);
        entry.age = turn - entry.turn;
        entry.turn = turn;
        entry.count += 1;
        prev_entry = *entry;
    }
    prev_num
}

fn main() {
    println!("{}", solve(&[20, 0, 1, 11, 6, 3], 2020));
    println!("{}", solve(&[20, 0, 1, 11, 6, 3], 30000000));
}

#[test]
fn test_sample() {
    assert_eq!(solve(&[0, 3, 6], 9), 4);
    assert_eq!(solve(&[0, 3, 6], 5), 3);
    assert_eq!(solve(&[0, 3, 6], 10), 0);
}

#[test]
fn test_simple() {
    assert_eq!(solve(&[0, 3, 6], 2020), 436);
}

#[test]
fn test_additional() {
    assert_eq!(solve(&[1, 3, 2], 2020), 1);
    assert_eq!(solve(&[2, 1, 3], 2020), 10);
    assert_eq!(solve(&[1, 2, 3], 2020), 27);
    assert_eq!(solve(&[2, 3, 1], 2020), 78);
    assert_eq!(solve(&[3, 2, 1], 2020), 438);
    assert_eq!(solve(&[3, 1, 2], 2020), 1836);
}
