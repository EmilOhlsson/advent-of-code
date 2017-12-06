use std::collections::HashSet;

fn loop_length(mut banks: Vec<usize>) -> usize {
    let mut set = HashSet::new();
    set.insert(banks.clone());
    let l = banks.len();
    loop {
        let (i, v) = banks.iter().enumerate()
            .fold((0, 0), |m, (i, &v)| if v > m.1 { (i, v) } else { m });
        banks[i] = 0;
        for d in 0..v {
            banks[(i + d + 1) % l] += 1;
        }
        if set.contains(&banks) { break; }
        set.insert(banks.clone());
    }
    return set.len();
}

#[test]
fn test_loop_length() {
    assert_eq!(loop_length(vec![0, 2, 7, 0]), 5);
}

fn main() {
    let input = include_str!("input");
    let banks = input.split_whitespace()
        .map(|t| t.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    println!("{}", loop_length(banks));
}
