use std::collections::{HashMap, HashSet};

fn solve_p1(input: &str) -> i32 {
    let mut wires = Vec::new();
    for line in input.lines() {
        let mut points = HashSet::new();
        let mut head = (0i32, 0i32);
        for tok in line.split(',') {
            let dir_str = tok.get(0..1).unwrap();
            let dist = tok.get(1..).unwrap().parse::<i32>().unwrap();
            let dir = match dir_str {
                "L" => (-1, 0),
                "R" => (1, 0),
                "U" => (0, 1),
                "D" => (0, -1),
                _ => panic!("Invalid direction"),
            };
            for _ in 0..dist {
                head = (head.0 + dir.0, head.1 + dir.1);
                points.insert(head);
            }
        }
        wires.push(points);
    }

    wires[0]
        .intersection(&wires[1])
        .map(|&(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

fn solve_p2(input: &str) -> i32 {
    let mut wires = Vec::new();
    let mut dists = Vec::new();
    for line in input.lines() {
        let mut steps = 0;
        let mut points = HashSet::new();
        let mut ds = HashMap::new();
        let mut head = (0i32, 0i32);
        for tok in line.split(',') {
            let dir_str = tok.get(0..1).unwrap();
            let dist = tok.get(1..).unwrap().parse::<i32>().unwrap();
            let dir = match dir_str {
                "L" => (-1, 0),
                "R" => (1, 0),
                "U" => (0, 1),
                "D" => (0, -1),
                _ => panic!("Invalid direction"),
            };
            for _ in 0..dist {
                head = (head.0 + dir.0, head.1 + dir.1);
                steps += 1;
                points.insert(head);
                ds.insert(head, steps);
            }
        }
        wires.push(points);
        dists.push(ds);
    }

    wires[0]
        .intersection(&wires[1])
        .map(|p| dists[0].get(p).unwrap() + dists[1].get(p).unwrap())
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve_p1(input));
    println!("{}", solve_p2(input));
}

#[test]
fn test() {
    let test0 = include_str!("input-test0");
    let test1 = include_str!("input-test1");
    assert_eq!(solve_p1(test0), 159);
    assert_eq!(solve_p1(test1), 135);
    assert_eq!(solve_p2(test0), 610);
    assert_eq!(solve_p2(test1), 410);
}
