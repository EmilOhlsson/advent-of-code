fn metasum(nums: &Vec<usize>, index: &mut usize, d: usize) -> usize {
    let child = nums[*index];
    let ents = nums[*index + 1];
    let mut sum = 0;

    println!("{:width$}Node at {} has {} entries and {} children", "", index, ents, child, width = d);
    *index += 2;
    for _ in 0..child {
        sum += metasum(nums, index, d + 1);
    }
    println!("{:width$}Adding nodes: {:?}", "", &nums[*index..*index+ents], width = d);
    for _ in 0..ents {
        sum += nums[*index];
        *index += 1;
    }

    sum
}

fn solve(input: &str) -> usize {
    let nums = input.split_whitespace().map(|t| t.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let mut index = 0;
    metasum(&nums, &mut index, 0)
}

fn main() {
    let input = include_str!("input").trim();
    let solution = solve(input);
    println!("{}", solution);
}

#[test]
fn test() {
    assert_eq!(solve("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), 138);
}
