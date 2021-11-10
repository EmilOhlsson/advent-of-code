type Cache = std::collections::HashMap<usize, u64>;

fn get_paths(value: usize, i_start: usize, adapters: &[usize], cache: &mut Cache) -> u64 {
    if i_start == adapters.len() {
        1
    } else if let Some(cached) = cache.get(&value) {
        *cached
    } else {
        let mut sum = 0;
        for (i_n, v_n) in adapters.iter().enumerate().skip(i_start).take(3) {
            if *v_n <= value + 3 {
                sum += get_paths(*v_n, i_n + 1, adapters, cache);
            }
        }
        cache.insert(value, sum);
        sum
    }
}

fn solve(input: &str) -> (u64, u64) {
    let mut adapters = input
        .lines()
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    adapters.sort_unstable();
    let mut diffs = vec![0, 1, 0, 1];
    for pair in adapters.windows(2) {
        diffs[pair[1] - pair[0]] += 1;
    }
    let part1 = diffs[1] * diffs[3];
    let mut cache = Cache::new();
    let part2 = get_paths(0, 0, &adapters, &mut cache);

    (part1, part2)
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input));
}

#[test]
fn test_simple() {
    let input = include_str!("input-simple");
    assert_eq!(solve(input), (7 * 5, 8));
}

#[test]
fn test_larger() {
    let input = include_str!("input-larger");
    assert_eq!(solve(input), (22 * 10, 19208));
}
