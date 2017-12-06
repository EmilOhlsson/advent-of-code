use std::collections::HashMap;

fn loop_length(mut banks: Vec<usize>) -> usize {
    let mut set = HashMap::new();
    let l = banks.len();
    for cycle in 0usize.. {
        set.insert(banks.clone(), cycle);
        let (i, v) = banks.iter().enumerate()
            .fold((0, 0), |m, (i, &v)| if v > m.1 { (i, v) } else { m });
        banks[i] = 0;
        for d in 0..v {
            banks[(i + d + 1) % l] += 1;
        }
        if let Some(prev_cycle) = set.get(&banks) {
            return cycle - prev_cycle + 1;
        }
    }
    panic!("fail!");
}

#[test]
fn test_loop_length() {
    assert_eq!(loop_length(vec![0, 2, 7, 0]), 4);
}

fn main() {
    let input = include_str!("input");
    let banks = input.split_whitespace()
        .map(|t| t.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    println!("{}", loop_length(banks));
}
