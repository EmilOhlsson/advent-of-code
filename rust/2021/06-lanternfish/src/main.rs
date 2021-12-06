fn solve(input: &str, days: u64) -> usize {
    let mut fishes = input
        .trim()
        .split(',')
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect::<Vec<u64>>();

    /* Naive approach */
    for _ in 0..days {
        let mut count = 0;
        for fish in fishes.iter_mut() {
            if *fish == 0 {
                count += 1;
                *fish = 6
            } else {
                *fish -= 1;
            }
        }
        for _ in 0..count {
            fishes.push(8);
        }
    }
    fishes.len()
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input, 80));
    println!("{}", solve(input, 256));
}

#[test]
fn test_simple() {
    let input = "3,4,3,1,2";
    assert_eq!(solve(input, 18), 26);
    assert_eq!(solve(input, 80), 5934);
    assert_eq!(solve(input, 256), 26984457539);
}
