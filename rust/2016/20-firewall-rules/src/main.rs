use std::cmp;

fn in_range(v: u64, r: &(u64, u64)) -> bool {
    r.0 <= v && v <= r.1
}

fn covers(a: &(u64, u64), b: &(u64, u64)) -> bool {
    a.0 <= b.0 && b.1 <= a.1
}

fn overlap(a: &(u64, u64), b: &(u64, u64)) -> Option<(u64, u64)> {
    if covers(a, b) || covers(b, a) || in_range(a.0, b) || in_range(a.1, b) {
        Some((cmp::min(a.0, b.0), cmp::max(a.1, b.1)))
    } else {
        None
    }
}

fn add_range(ranges: &mut Vec<(u64, u64)>, mut r: (u64, u64)) {
    let mut i = 0;

    while i < ranges.len() {
        if let Some(r_new) = overlap(&ranges[i], &r) {
            r = r_new;
            ranges.remove(i);
        } else {
            i += 1;
        }
    }
    ranges.push(r);
}

fn solve(ranges: &str, n: u64) -> u64 {
    let rs = ranges
        .lines()
        .map(|l: &str| {
            let mut lim = l.split('-').map(|t: &str| t.parse::<u64>().unwrap());
            (lim.next().unwrap(), lim.next().unwrap() + 1)
        }).collect::<Vec<(u64, u64)>>();

    let mut blacklist: Vec<(u64, u64)> = Vec::new();
    for (l, u) in rs.iter() {
        add_range(&mut blacklist, (*l, *u));
    }

    blacklist.sort();
    let mut whites = 0;
    let mut lp = 0;
    for r in blacklist.iter() {
        whites += r.0 - lp;
        lp = r.1;
    }

    println!("Found {} safe", whites);
    blacklist.iter().map(|v| v.1).min().unwrap()
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", solve(&input, std::u32::MAX.into()));
}
