use regex::Regex;
use std::collections::HashMap;

fn traverse<T: Default>(
    map: &mut HashMap<(u32, u32), T>,
    p0: (u32, u32),
    p1: (u32, u32),
    mutator: &dyn Fn(&mut T),
) {
    for x in p0.0..=p1.0 {
        for y in p0.1..=p1.1 {
            let light = map.entry((x, y)).or_insert_with(Default::default);
            mutator(light);
        }
    }
}

fn solve(input: &str) -> (usize, u32) {
    let mut map_p1: HashMap<(u32, u32), bool> = HashMap::new();
    let mut map_p2: HashMap<(u32, u32), u32> = HashMap::new();
    let re =
        Regex::new(r"(?m)^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    for cap in re.captures_iter(input) {
        let p0 = (
            cap[2].parse::<u32>().unwrap(),
            cap[3].parse::<u32>().unwrap(),
        );
        let p1 = (
            cap[4].parse::<u32>().unwrap(),
            cap[5].parse::<u32>().unwrap(),
        );
        traverse(
            &mut map_p1,
            p0,
            p1,
            match &cap[1] {
                "turn on" => &|light: &mut bool| {
                    *light = true;
                },
                "turn off" => &|light: &mut bool| {
                    *light = false;
                },
                "toggle" => &|light: &mut bool| {
                    *light = !*light;
                },
                _ => panic!("Unknown operation"),
            },
        );
        traverse(
            &mut map_p2,
            p0,
            p1,
            match &cap[1] {
                "turn on" => &|light: &mut u32| {
                    *light += 1;
                },
                "turn off" => &|light: &mut u32| {
                    if *light > 0 {
                        *light -= 1;
                    }
                },
                "toggle" => &|light: &mut u32| {
                    *light += 2;
                },
                _ => panic!("Unknown operation"),
            },
        );
    }
    (
        map_p1.values().filter(|&l| *l).count(),
        map_p2.values().sum(),
    )
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input));
}
