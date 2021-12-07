use itertools::Itertools;

fn triangle_sum(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn solve(input: &str) -> (i32, i32) {
    let nums = input
        .trim()
        .split(',')
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<i32>>();
    let (&lo, &hi) = nums.iter().minmax().into_option().unwrap();

    let part1 = (lo..=hi)
        .map(|p| nums.iter().map(|&n| (n - p).abs()).sum::<i32>())
        .min()
        .unwrap();

    let part2 = (lo..=hi)
        .map(|p| {
            nums.iter()
                .map(|&n| (n - p).abs())
                .map(triangle_sum)
                .sum::<i32>()
        })
        .min()
        .unwrap();

    (part1, part2)
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input));
}

#[test]
fn test() {
    let input = "16,1,2,0,4,2,7,1,2,14";
    assert_eq!(solve(input), (37, 168));
}
