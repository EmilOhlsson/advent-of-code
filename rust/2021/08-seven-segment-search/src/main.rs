use std::collections::HashSet;

fn solve_p1(input: &str) -> u32 {
    let mut count = 0;
    for line in input.lines() {
        let disp = line.split('|').nth(1).unwrap().trim();
        let digs = disp.split_whitespace().collect::<Vec<&str>>();
        for dig in digs {
            count += match dig.len() {
                2 => 1, /* 1 */
                3 => 1, /* 7 */
                4 => 1, /* 4 */
                7 => 1, /* 8 */
                _ => 0, /* ? */
            }
        }
    }
    count
}

fn solve_p2(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        /* Yuck... this is messy and very wasteful. This could be done with bitmasks instead */
        let parts = line.split('|').map(str::trim).collect::<Vec<&str>>();
        let samples = &parts[0].split_whitespace().collect::<Vec<&str>>();
        let samples = samples
            .iter()
            .map(|s| s.chars().collect::<HashSet<char>>())
            .collect::<Vec<HashSet<char>>>();
        let digs = parts[1].split_whitespace().collect::<Vec<&str>>();
        let digs = digs
            .iter()
            .map(|s| s.chars().collect::<HashSet<char>>())
            .collect::<Vec<HashSet<char>>>();
        let mut table = vec![HashSet::<char>::new(); 10];

        /* Update table for 1, 4, 7 and 8 */
        for sample in samples.iter() {
            match sample.len() {
                2 => table[1] = sample.clone(),
                3 => table[7] = sample.clone(),
                4 => table[4] = sample.clone(),
                7 => table[8] = sample.clone(),
                _ => (),
            };
        }

        /* Use 1, 3, 7 and 8 to identify rest */
        while table.iter().any(|t| t.is_empty()) {
            for sample in samples.iter() {
                /* 2, 3, 5 have 5 segments */
                if sample.len() == 5 {
                    if table[3].is_empty() {
                        if table[7].is_subset(sample) {
                            table[3] = sample.clone();
                        }
                    }
                    if !table[9].is_empty() {
                        let five_mask = table[9]
                            .difference(&table[1])
                            .cloned()
                            .collect::<HashSet<char>>();
                        if table[2].is_empty() {
                            if !five_mask.is_subset(sample) && !table[1].is_subset(sample) {
                                table[2] = sample.clone();
                            }
                        }
                        if table[5].is_empty() {
                            if five_mask.is_subset(sample) && !table[1].is_subset(sample) {
                                table[5] = sample.clone();
                            }
                        }
                    }
                }

                /* 6, 9 and 0 have 6 segments */
                if sample.len() == 6 && !table[3].is_empty() {
                    if table[9].is_empty() {
                        if table[7].is_subset(sample) && table[3].is_subset(sample) {
                            table[9] = sample.clone();
                        }
                    }
                    if table[0].is_empty() {
                        if table[7].is_subset(sample) && !table[3].is_subset(sample) {
                            table[0] = sample.clone();
                        }
                    }
                    if table[6].is_empty() {
                        if !table[7].is_subset(sample) {
                            table[6] = sample.clone();
                        }
                    }
                }
            }
        }

        /* Use table to match displayed value */
        let mut value = 0;
        for digit in &digs {
            for (v, segments) in table.iter().enumerate() {
                if digit == segments {
                    value = value * 10 + v;
                }
            }
        }
        sum += value;
    }
    sum
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve_p1(input));
    println!("{}", solve_p2(input));
}

#[test]
fn test_simple() {
    let input = include_str!("input-simple");
    assert_eq!(solve_p1(input), 0);
    assert_eq!(solve_p2(input), 5353);
}

#[test]
fn test_sample() {
    let input = include_str!("input-sample");
    assert_eq!(solve_p1(input), 26);
    assert_eq!(solve_p2(input), 61229);
}
