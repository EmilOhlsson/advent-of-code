use coding_challenge_utils::sets::permutations;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

fn solve(input: &str) -> (u32, u32) {
    let re = Regex::new(r"^(\w+) to (\w+) = (\d+)$").unwrap();
    let mut distances: HashMap<String, HashMap<String, u32>> = HashMap::new();
    let mut destinations: HashSet<String> = HashSet::new();
    input
        .lines()
        .map(|line| re.captures(line).unwrap())
        .for_each(|cap| {
            destinations.insert(cap[1].to_string());
            destinations.insert(cap[2].to_string());

            let dest = distances
                .entry(cap[1].to_string())
                .or_insert_with(HashMap::new);
            dest.insert(cap[2].to_string(), cap[3].parse::<u32>().unwrap());
            let dest = distances
                .entry(cap[2].to_string())
                .or_insert_with(HashMap::new);
            dest.insert(cap[1].to_string(), cap[3].parse::<u32>().unwrap());
        });
    let mut dest = destinations.into_iter().collect::<Vec<String>>();
    dest.sort();
    let mut ids = (0..dest.len()).collect::<Vec<usize>>();
    let routes = permutations(&mut ids);

    let dists = routes
        .iter()
        .map(|route| {
            let mut dist = 0;
            print!("{}", dest[route[0]]);
            for i in 1..route.len() {
                print!(" -> {}", dest[route[i]]);
                dist += distances
                    .get(&dest[route[i]])
                    .unwrap()
                    .get(&dest[route[i - 1]])
                    .unwrap();
            }
            println!(" = {}", dist);
            dist
        })
        .collect::<Vec<u32>>();
    (*dists.iter().min().unwrap(), *dists.iter().max().unwrap())
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input));
}

#[test]
fn test() {
    let input = include_str!("input-test");
    assert_eq!(solve(&input), (605, 982));
}
