use std::collections::HashMap;

fn main() {
    let mut registers = HashMap::<&str, isize>::new();
    let mut max_held = 0;
    let input = include_str!("input");
    for l in input.lines() {
        let t = l.split_whitespace().collect::<Vec<&str>>();
        let mut inc = t[2].parse::<isize>().unwrap();
        if t[1] == "dec" {inc = -inc}
        let cmp_r = {
            let val = registers.entry(t[4]).or_insert(0);
            *val
        };
        let fst = registers.entry(t[0]).or_insert(0);
        let cmp_v = t[6].parse::<isize>().unwrap();
        let expr = match t[5] {
            ">" => cmp_r > cmp_v,
            "<" => cmp_r < cmp_v,
            ">=" => cmp_r >= cmp_v,
            "<=" => cmp_r <= cmp_v,
            "!=" => cmp_r != cmp_v,
            "==" => cmp_r == cmp_v,
            _ => panic!("Aaaaaargh: {}", t[5]),
        };
        if expr {
            *fst += inc;
            if *fst > max_held {
                max_held = *fst;
            }
        }
    }
    let max = registers.values().max().unwrap();
    println!("{}, {}", max, max_held);
}
