use regex;
use regex::Regex;
use std::cmp::min;

fn get_req(mass: u32) -> u32 {
    mass / 3 - min(2, mass / 3)
}

fn get_req_v2(mut mass: u32) -> u32 {
    let mut sum = 0;
    while mass > 0 {
        sum += get_req(mass);
        mass = get_req(mass);
    }

    sum
}

fn solve(input: &str, fuel_req: &dyn Fn(u32) -> u32) -> u32 {
    let re = Regex::new(r"^(\d+)$").unwrap();
    input
        .lines()
        .map(|line| re.captures(line).unwrap())
        .map(|cap| cap[1].parse::<u32>().unwrap())
        .map(fuel_req)
        .sum()
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input, &get_req));
    println!("{}", solve(input, &get_req_v2));
}

#[test]
fn test() {
    assert_eq!(get_req(12), 2);
    assert_eq!(get_req_v2(100756), 50346);
}
