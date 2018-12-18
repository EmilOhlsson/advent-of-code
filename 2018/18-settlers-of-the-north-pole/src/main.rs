use coding_challenge_utils::coord::Cartesian;

use std::collections::HashMap;

type Area = HashMap<Cartesian, char>;

fn count_adjacent(area: &Area, coord: Cartesian, acre: &char) -> usize {
    coord
        .neigh8()
        .iter()
        .filter_map(|cn| area.get(cn))
        .filter(|an| an == &acre)
        .count()
}

fn str_to_area(input: &str) -> Area {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.trim().chars().enumerate().map(move |(x, ch)| {
                (
                    Cartesian::new(x as i32, y as i32),
                    match ch {
                        '.' | '#' | '|' => ch,
                        _ => panic!("Does not recognize {:?}", ch),
                    },
                )
            })
        })
        .collect::<Area>()
}

fn area_to_str(area: &Area, size: usize) -> String {
    let mut res = String::new();
    for y in 0..size {
        for x in 0..size {
            res.push(*area.get(&Cartesian::new(x as i32, y as i32)).unwrap_or(&'?'));
        }
        res.push('\n');
    }
    res
}

fn score(area: &str) -> usize {
    area.chars().filter(|a| a == &'|').count() * area.chars().filter(|a| a == &'#').count()
}

fn transform(area: &Area) -> Area {
    area.iter()
        .map(|(c, a)| {
            (
                *c,
                match a {
                    '.' => {
                        if count_adjacent(&area, *c, &'|') >= 3 {
                            '|'
                        } else {
                            '.'
                        }
                    }
                    '|' => {
                        if count_adjacent(&area, *c, &'#') >= 3 {
                            '#'
                        } else {
                            '|'
                        }
                    }
                    '#' => {
                        if count_adjacent(&area, *c, &'#') >= 1
                            && count_adjacent(&area, *c, &'|') >= 1
                        {
                            '#'
                        } else {
                            '.'
                        }
                    }
                    _ => panic!("Does not recognize {:?}", a),
                },
            )
        })
        .collect::<Area>()
}

fn solve_p1(input: &str, size: usize, minutes: usize) -> usize {
    let mut area = input.to_owned();
    let mut lookup: HashMap<String, String> = HashMap::new();

    for i in 0..minutes {
        let new_area = lookup.entry(area.clone()).or_insert({
            let rep = str_to_area(&area);
            area_to_str(&transform(&rep), size)
        });

        area = new_area.clone();
        println!("{}: {}", i, score(&area));
    }

    score(&area)
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", solve_p1(input, 50, 10));
    println!("{}", solve_p1(input, 50, 1_000_000_000));
    // Should not really run to finish. The repeats every 2041 - 2013 = 28th line
    // so: (1_000_000_000 -  2013) % 28 = 23. So 23 steps from 2013 the answer is
    // -- 189720 -- 
}

#[test]
fn test() {
    let input = include_str!("input-simple.txt");
    assert_eq!(solve_p1(input, 10, 10), 1147);
}
