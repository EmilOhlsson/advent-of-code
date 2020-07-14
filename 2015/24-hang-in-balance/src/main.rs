use itertools::Itertools;

fn solve(input: &str, groups: u128) -> u128 {
    let weights = input
        .lines()
        .map(str::parse::<u128>)
        .map(Result::unwrap)
        .collect::<Vec<u128>>();
    let group_size = weights.iter().sum::<u128>() / groups;
    let mut answer = std::u128::MAX;
    for i in 0..weights.len() {
        let mut found = false;
        for cmb in weights
            .iter()
            .combinations(i)
            .filter(|list| list.iter().cloned().sum::<u128>() == group_size)
        {
            found = true;
            let qe = cmb.iter().cloned().cloned().product::<u128>();
            answer = std::cmp::min(qe, answer);
        }
        if found {
            break;
        }
    }
    answer
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input, 3));
    println!("{}", solve(input, 4));
}
