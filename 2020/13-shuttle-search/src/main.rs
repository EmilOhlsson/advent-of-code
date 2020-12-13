fn div_ceil(x: u64, y: u64) -> u64 {
    (x + y - 1) / y
}

fn round_up(x: u64, y: u64) -> u64 {
    div_ceil(x, y) * y
}

fn solve_p1(input: &str) -> u64 {
    let mut lines = input.lines();
    let time = lines.next().unwrap().parse::<u64>().unwrap();
    let pair = lines
        .next()
        .unwrap()
        .split(',')
        .map(str::parse::<u64>)
        .filter_map(Result::ok)
        .map(|d| (d, round_up(time, d) - time))
        .min_by_key(|(_, v)| *v)
        .unwrap();
    pair.0 * pair.1
}

fn solve_p2(input: &str) -> u64 {
    let mut lines = input.lines();
    lines.next(); // Skip time
    let res_mod = lines
        .next()
        .unwrap()
        .split(',')
        .map(str::parse::<u64>)
        .enumerate()
        .filter_map(|(i, v)| {
            if let Ok(d) = v {
                Some((i as u64, d))
            } else {
                None
            }
        });

    let mut answer = 0;
    let mut least_div = 1;
    for (r, m) in res_mod {
        while (answer + r) % m != 0 {
            answer += least_div;
        }
        least_div *= m;
    }
    answer
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve_p1(input));
    println!("{}", solve_p2(input));
}

#[test]
fn test() {
    let input = include_str!("input-simple");
    assert_eq!(solve_p1(input), 295);
    assert_eq!(solve_p2(input), 1068781);
}
