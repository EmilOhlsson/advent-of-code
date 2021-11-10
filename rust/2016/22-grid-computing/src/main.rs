use regex::Regex;
use std::cmp;
use std::collections::HashMap;

fn solve(input: &str) -> u32 {
    let mut nodes = HashMap::<(i32, i32), (i32, i32, i32)>::new();
    let mut xs = (std::i32::MAX, std::i32::MIN);
    let mut ys = (std::i32::MAX, std::i32::MIN);
    let re = Regex::new(r"node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T").unwrap();
    for cap in input.lines().filter_map(|line| re.captures(line)) {
        let nums = cap
            .iter()
            .skip(1)
            .map(Option::unwrap)
            .map(|m| m.as_str())
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .collect::<Vec<i32>>();
        nodes.insert((nums[0], nums[1]), (nums[2], nums[3], nums[4]));
        xs = (cmp::min(xs.0, nums[0]), cmp::max(xs.1, nums[0]));
        ys = (cmp::min(ys.0, nums[1]), cmp::max(ys.1, nums[1]));
    }

    println!("Grid: {:?}-{:?}", xs, ys);

    print!("{:9}", "");
    for x in xs.0..=xs.1 {
        print!("x={:2}     ", x);
    }
    println!();
    for y in ys.0..=ys.1 {
        print!("y={:2}  ", y);
        for x in xs.0..=xs.1 {
            let node = nodes.get(&(x, y)).unwrap();
            let mut node_desc = format!("{}/{}", node.1, node.0);
            node_desc = format!("{:>7}", node_desc);
            if node.1 == 0 {
                node_desc = format!("\x1b[4m{}\x1b[0m", node_desc);
            }
            print!("{}  ", node_desc);
        }
        println!();
        println!();
    }

    // size -- used -- avail
    let mut viable_pairs = 0;
    for (pos_a, node_a) in &nodes {
        if node_a.1 > 0 {
            for (pos_b, node_b) in &nodes {
                if pos_a != pos_b {
                    if node_a.1 <= node_b.2 {
                        viable_pairs += 1;
                    }
                }
            }
        }
    }
    viable_pairs
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input));
    println!(
        "\
move three left, move up 20, move right 34, circle left 33 * 5 steps = {}
",
        3 + 20 + 34 + 33 * 5
    );
}
