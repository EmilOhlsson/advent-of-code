use std::collections::HashSet;

fn parse(input: &str) -> HashSet<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars().enumerate().filter_map(move |(c, ch)| {
                if ch == '#' {
                    Some((r as i32, c as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn tick(lights: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut next = HashSet::new();
    for r in 0..100 {
        for c in 0..100 {
            let mut count = 0;
            for (dr, dc) in &[
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                if lights.contains(&(r + dr, c + dc)) {
                    count += 1;
                }
            }
            let is_on = lights.contains(&(r, c));
            let stays_on = is_on && (count == 2 || count == 3);
            let turns_on = !is_on && count == 3;
            if stays_on || turns_on {
                next.insert((r, c));
            }
        }
    }
    next
}

fn solve(input: &str) -> u32 {
    let mut lights = parse(input);

    for _ in 0..100 {
        let next = tick(&lights);
        lights = next;
    }
    lights.len() as u32
}

fn tick_v2(lights: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut next = HashSet::new();
    for r in 0..100 {
        for c in 0..100 {
            let mut count = 0;
            for (dr, dc) in &[
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                if lights.contains(&(r + dr, c + dc)) {
                    count += 1;
                }
            }
            let is_on = lights.contains(&(r, c));
            let stays_on = is_on && (count == 2 || count == 3);
            let turns_on = !is_on && count == 3;
            if stays_on || turns_on {
                next.insert((r, c));
            }
        }
    }
    next.insert((0, 0));
    next.insert((0, 99));
    next.insert((99, 0));
    next.insert((99, 99));
    next
}

fn solve_v2(input: &str) -> u32 {
    let mut lights = parse(input);

    for _ in 0..100 {
        let next = tick_v2(&lights);
        lights = next;
    }
    lights.len() as u32
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input));
    println!("{}", solve_v2(input));
}
