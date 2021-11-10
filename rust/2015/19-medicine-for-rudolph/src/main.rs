use std::collections::HashSet;

fn parse(input: &str) -> Vec<(String, String)> {
    let mut rules = Vec::new();
    for line in input.lines() {
        let mut toks = line.split("=>").map(str::trim).map(str::to_string);
        rules.push((toks.next().unwrap(), toks.next().unwrap()));
    }
    rules
}

fn get_transforms(molecule: &str, (src, dst): (&str, &str)) -> Vec<String> {
    molecule
        .match_indices(src)
        .map(|(i, _)| {
            let mut mol = molecule.to_string();
            mol.replace_range(i..(i + src.len()), dst);
            mol
        })
        .collect()
}

fn get_all_transforms(molecule: &str, rules: &[(String, String)]) -> HashSet<String> {
    let mut results = HashSet::<String>::new();
    for (src, dst) in rules {
        for res in get_transforms(molecule, (src, dst)) {
            results.insert(res);
        }
    }
    results
}

fn solve(input: &str, molecule: &str) -> u32 {
    let rules = parse(input);
    get_all_transforms(molecule, &rules).len() as u32
}

fn solve_v2(molecule: &str) -> usize {
    let syms = molecule.chars().filter(|ch| ch.is_uppercase()).count();
    let rns = molecule.matches("Rn").count();
    let ars = molecule.matches("Ar").count();
    let ys = molecule.matches('Y').count();
    syms - rns - ars - 2 * ys - 1
}

fn main() {
    let input = include_str!("input");
    let molecule = include_str!("input-molecule");
    println!("{}", solve(input, molecule));
    println!("{}", solve_v2(molecule));
}

#[test]
fn test() {
    let input = include_str!("input-simple");
    assert_eq!(solve(input, "HOH"), 4);
    assert_eq!(solve(input, "HOHOHO"), 7);
}
