use std::collections::HashMap;

type SueMap = HashMap<String, HashMap<String, u32>>;

fn parse(input: &str) -> SueMap {
    input
        .lines()
        .map(|line| {
            let mut split = line.splitn(2, ':').map(str::trim);
            (split.next().unwrap().to_string(), {
                split
                    .next()
                    .unwrap()
                    .split(',')
                    .map(str::trim)
                    .map(|tok| {
                        let mut split = tok.split(':').map(str::trim);
                        (
                            split.next().unwrap().to_string(),
                            split.next().unwrap().parse::<u32>().unwrap(),
                        )
                    })
                    .collect::<HashMap<String, u32>>()
            })
        })
        .collect::<SueMap>()
}

fn solve(input: &str) {
    let real_sue = [
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<String, u32>>();
    let sue_map = parse(input);

    for (sue, pets) in sue_map {
        let mut candidate = true;
        for (pet, count) in &real_sue {
            if let Some(c) = pets.get(pet) {
                candidate = c == count;
            }
            if !candidate {
                break;
            }
        }
        if candidate {
            println!("candidate Sue: {}", sue);
        }
    }
}

fn solve_v2(input: &str) {
    let real_sue = [
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<String, u32>>();
    let sue_map = parse(input);

    for (sue, pets) in sue_map {
        let mut candidate = true;
        for (pet, count) in &real_sue {
            if let Some(c) = pets.get(pet) {
                candidate = match pet.as_str() {
                    "cats" | "trees" => c > count,
                    "pomeranians" | "goldfish" => c < count,
                    _ => c == count,
                };
            }

            if !candidate {
                break;
            }
        }
        if candidate {
            println!("real candidate Sue: {}", sue);
        }
    }
}

fn main() {
    let input = include_str!("input");
    solve(input);
    solve_v2(input);
}
