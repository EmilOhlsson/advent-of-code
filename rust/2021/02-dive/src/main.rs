use regex::Regex;

fn solve_p1(input: &str) -> i32 {
    let re = Regex::new(r"^(\w+) (\d+)$").unwrap();
    let mut depth = 0;
    let mut distance = 0;
    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        let amount = cap[2].parse::<i32>().unwrap();
        match &cap[1] {
            "forward" => distance += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => panic!("Unknown directive"),
        }
    }

    depth * distance
}

fn solve_p2(input: &str) -> i32 {
    let re = Regex::new(r"^(\w+) (\d+)$").unwrap();
    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;
    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        let amount = cap[2].parse::<i32>().unwrap();
        match &cap[1] {
            "forward" => {
                distance += amount;
                depth += aim * amount
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => panic!("Unknown directive"),
        }
    }

    depth * distance
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve_p1(input));
    println!("{}", solve_p2(input));
}
