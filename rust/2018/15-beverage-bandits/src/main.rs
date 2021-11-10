use coding_challenge_utils::coord::Cartesian as C2d;
use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
enum Score {
    Elf(i32),
    ElfWithLosses(i32),
    Goblin(i32),
}

#[derive(Copy, Clone)]
struct Being {
    power: i32,
    health: i32,
    moved: i32,
}

impl Being {
    fn new(power: i32, health: i32) -> Being {
        Being {
            power,
            health,
            moved: 0,
        }
    }
}

fn reconstruct_path(mut dst: C2d, came_from: &HashMap<C2d, C2d>) -> Vec<C2d> {
    let mut path = vec![dst];
    while let Some(prev) = came_from.get(&dst) {
        path.push(*prev);
        dst = *prev;
    }
    path.pop();
    return path;
}

fn _print_distances(distances: &HashMap<C2d, usize>, area: (i32, i32), pos: C2d) {
    for y in 0..=area.1 {
        for x in 0..=area.0 {
            if x == pos.x && y == pos.y {
                print!("XX ");
            } else {
                print!(
                    "{:2} ",
                    if let Some(d) = distances.get(&C2d::new(x, y)) {
                        format!("{}", d)
                    } else {
                        format!("--")
                    }
                );
            }
        }
        println!();
    }
    println!();
}

fn neigh(c: C2d) -> Vec<C2d> {
    vec![
        c + C2d::new(0, -1),
        c + C2d::new(-1, 0),
        c + C2d::new(1, 0),
        c + C2d::new(0, 1),
    ]
}

fn find_target(
    src: &C2d,
    dst: &HashMap<C2d, Being>,
    team: &HashMap<C2d, Being>,
    walls: &HashSet<C2d>,
    _area: (i32, i32),
) -> Option<Vec<C2d>> {
    let mut queue: VecDeque<C2d> = VecDeque::new();
    let mut path: HashMap<C2d, C2d> = HashMap::new();
    let mut visited: HashSet<C2d> = HashSet::new();
    let mut distances: HashMap<C2d, usize> = HashMap::new();
    let is_obstacle =
        |being: &C2d| dst.contains_key(being) || team.contains_key(being) || walls.contains(being);

    queue.push_back(*src);
    distances.insert(*src, 0);
    visited.insert(*src);
    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();
        let current_dist = *distances.get(&current).unwrap();

        for n in neigh(current) {
            if !visited.contains(&n) && !is_obstacle(&n) {
                distances.insert(n, current_dist + 1);
                visited.insert(n);
                if !path.contains_key(&n) {
                    assert!(path.insert(n, current).is_none());
                }
                queue.push_back(n);
            }
        }
    }

    // _print_distances(&distances, _area, *src);

    // Pick target in reading order
    let mut targets: Vec<_> = dst
        .keys()
        .flat_map(|c| c.neigh4().into_iter())
        .filter_map(|c| {
            if let Some(dist) = distances.get(&c) {
                Some((dist, (c.y, c.x)))
            } else {
                None
            }
        })
        .collect();
    targets.sort();

    //println!("{:?} -- {:?}", src, targets);
    //_print_distances(&distances, _area);

    if let Some((_, (y, x))) = targets.first() {
        Some(reconstruct_path(C2d::new(*x, *y), &path))
    } else {
        None
    }
}

fn _print_board(
    walls: &HashSet<C2d>,
    elfs: &HashMap<C2d, Being>,
    goblins: &HashMap<C2d, Being>,
    area: (i32, i32),
) {
    for y in 0..=area.1 {
        let mut status: Vec<(char, i32)> = Vec::new();
        for x in 0..=area.0 {
            let c = C2d::new(x, y);
            print!("{}", {
                if walls.contains(&c) {
                    '#'
                } else if let Some(elf) = elfs.get(&c) {
                    status.push(('E', elf.health));
                    'E'
                } else if let Some(goblin) = goblins.get(&c) {
                    status.push(('G', goblin.health));
                    'G'
                } else {
                    '.'
                }
            });
        }
        println!(
            "\t{}",
            status.iter().fold(String::new(), |st, (ch, h)| format!(
                "{} ({}, {}), ",
                st, ch, h
            ))
        );
    }
    println!();
}

fn try_attack(_desc: char, pos: C2d, power: i32, others: &mut HashMap<C2d, Being>) -> bool {
    if let Some((target, _)) = neigh(pos)
        .iter()
        .filter_map(|n| {
            if let Some(other) = others.get(&n) {
                Some((*n, other.health))
            } else {
                None
            }
        })
        .min_by_key(|(_, h)| *h)
    {
        let mut other = others.remove(&target).unwrap();
        other.health -= power;
        if other.health > 0 {
            others.insert(target, other);
        }
        true
    } else {
        false
    }
}

fn being_turn(
    _desc: char,
    round: i32,
    c: C2d,
    team: &mut HashMap<C2d, Being>,
    others: &mut HashMap<C2d, Being>,
    walls: &HashSet<C2d>,
    area: (i32, i32),
) {
    if let Some((mut pos, mut being)) = team.remove_entry(&c) {
        if being.moved == round {
            team.insert(c, being);
            return;
        }
        being.moved = round;

        if !try_attack(_desc, pos, being.power, others) {
            if let Some(target_path) = find_target(&c, &others, &team, &walls, area) {
                // Take one step
                if target_path.len() > 0 {
                    pos = *target_path.last().unwrap();
                }
                try_attack(_desc, pos, being.power, others);
            }
        }

        // Done, reinsert
        team.insert(pos, being);
    }
}

fn solve(input: &str, elf_power: i32) -> Score {
    let mut area = (0i32, 0i32);
    let mut walls: HashSet<C2d> = HashSet::new();
    let mut goblins: HashMap<C2d, Being> = HashMap::new();
    let mut elfs: HashMap<C2d, Being> = HashMap::new();
    let mut coords: Vec<C2d> = Vec::new();

    // Build data set
    for (y, l) in input.lines().enumerate() {
        for (x, ch) in l.trim().chars().enumerate() {
            area = (max(x as i32, area.0), max(y as i32, area.1));
            let coord = C2d::new(x as i32, y as i32);
            match ch {
                '#' => {
                    walls.insert(coord);
                }
                'G' => {
                    goblins.insert(coord, Being::new(3, 200));
                    coords.push(coord);
                }
                'E' => {
                    elfs.insert(coord, Being::new(elf_power, 200));
                    coords.push(coord);
                }
                _ => {
                    coords.push(coord);
                }
            }
        }
    }
    let elf_count = elfs.len();

    // Battle it out!
    //_print_board(&walls, &elfs, &goblins, area);
    for round in 1i32.. {
        for c in coords.iter() {
            being_turn('E', round, *c, &mut elfs, &mut goblins, &walls, area);
            being_turn('G', round, *c, &mut goblins, &mut elfs, &walls, area);
        }

        //_print_board(&walls, &elfs, &goblins, area);
        let healthsum = |set: &HashMap<C2d, Being>| set.values().map(|e| e.health).sum::<i32>();

        if elfs.len() == 0 {
            let hsum = healthsum(&goblins);
            println!(
                "goblins win on round {}, with a total health of {}",
                round, hsum
            );
            return Score::Goblin(hsum * (round - 1));
        }
        if goblins.len() == 0 {
            let hsum = healthsum(&elfs);
            println!("elfs win on round {} with total health of {}", round, hsum);
            if elfs.len() < elf_count {
                return Score::ElfWithLosses(hsum * (round - 1));
            } else {
                return Score::Elf(hsum * (round - 1));
            }
        }
    }
    panic!();
}

fn main() {
    let mut power = 3;
    let input = include_str!("input.txt"); // sometimes calculates one turn too many
    println!("{:?}", solve(input, power));

    loop {
        power += 1;
        let result = solve(input, power);
        if let Score::Elf(points) = result {
            println!(
                "Elfs win if they have {} attack power, score is {}",
                power, points
            ); // 15040, 15200 is too low
            break;
        }
    }
}

#[test]
fn test0() {
    let input = include_str!("input-simple-0.txt");
    assert_eq!(solve(input), 27730);
}

// Movement test
//#[test]
//fn test1() {
//    let input = include_str!("input-simple-1.txt");
//    assert_eq!(solve(input), 29374);
//}

#[test]
fn test4() {
    let input = include_str!("input-simple-4.txt");
    assert_eq!(solve(input), 18740);
}
