fn metasum(nums: &Vec<usize>, index: &mut usize, d: usize) -> usize {
    let child = nums[*index];
    let ents = nums[*index + 1];
    let mut sum = 0;

    *index += 2;
    for _ in 0..child {
        sum += metasum(nums, index, d + 1);
    }
    for _ in 0..ents {
        sum += nums[*index];
        *index += 1;
    }

    sum
}

fn metaval(nums: &Vec<usize>, index: &mut usize, d: usize) -> usize {
    let child = nums[*index];
    let ents = nums[*index + 1];
    let mut val = 0;
    let mut children = vec![0; child];

    println!(
        "{:width$}Node@{}: c={}, e={}",
        "",
        index,
        child,
        ents,
        width = d
    );

    *index += 2;
    for c in 0..child {
        children[c] = metaval(nums, index, d + 1);
    }
    for _ in 0..ents {
        if child == 0 {
            val += nums[*index];
        } else {
            val += children.get(nums[*index] - 1).unwrap_or(&0);
        }
        *index += 1;
    }

    val
}

fn solve_p1(input: &str) -> usize {
    let nums = input
        .split_whitespace()
        .map(|t| t.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut index = 0;
    metasum(&nums, &mut index, 0)
}

fn solve_p2(input: &str) -> usize {
    let nums = input
        .split_whitespace()
        .map(|t| t.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut index = 0;
    metaval(&nums, &mut index, 0)
}
fn main() {
    let input = include_str!("input").trim();
    println!("{}", solve_p1(input));
    println!("{}", solve_p2(input));
}

#[test]
fn test() {
    assert_eq!(solve_p1("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), 138);
    assert_eq!(solve_p2("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), 66);
}
