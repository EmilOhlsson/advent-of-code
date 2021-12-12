use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Cavern<'a> {
    Start,
    Small(&'a str),
    Large(&'a str),
    End,
}

fn to_cavern(s: &str) -> Cavern {
    match s {
        "start" => Cavern::Start,
        "end" => Cavern::End,
        _ => {
            if s.chars().next().unwrap().is_uppercase() {
                Cavern::Large(s)
            } else {
                Cavern::Small(s)
            }
        }
    }
}

impl<'a> Cavern<'a> {
    fn is_small(&self) -> bool {
        matches!(self, Cavern::Small(_))
    }
}

type Mapping<'a> = HashMap<Cavern<'a>, Vec<Cavern<'a>>>;
type Visited<'a> = HashSet<Cavern<'a>>;
type PathCount<'a> = HashMap<Cavern<'a>, u32>;

fn parse(input: &str) -> Mapping {
    let mut mapping = Mapping::new();
    for line in input.lines() {
        let mut toks = line.split('-');
        let a = to_cavern(toks.next().unwrap());
        let b = to_cavern(toks.next().unwrap());
        let from_a = mapping.entry(a).or_insert_with(Vec::new);
        from_a.push(b);
        let from_b = mapping.entry(b).or_insert_with(Vec::new);
        from_b.push(a);
    }
    mapping
}

fn solve_p1(input: &str) -> u32 {
    let mapping = parse(input);

    let mut path_count = PathCount::new();
    let mut visit_queue = VecDeque::with_capacity(1024);
    visit_queue.push_back((Cavern::Start, Visited::new()));

    while let Some((cavern, visited)) = visit_queue.pop_front() {
        let path_count = path_count.entry(cavern).or_insert(0);
        *path_count += 1;
        if cavern != Cavern::End {
            for next in mapping.get(&cavern).unwrap() {
                if !visited.contains(next) && *next != Cavern::Start {
                    let mut visited_new = visited.clone();
                    if cavern.is_small() {
                        visited_new.insert(cavern);
                    }
                    visit_queue.push_back((*next, visited_new));
                }
            }
        }
    }

    *path_count.get(&Cavern::End).unwrap()
}

fn solve_p2(input: &str) -> u32 {
    let mapping = parse(input);

    let mut path_count = PathCount::new();
    let mut visit_queue = VecDeque::with_capacity(1024);
    visit_queue.push_back((Cavern::Start, Visited::new(), false));

    while let Some((cavern, mut visited, has_revisited)) = visit_queue.pop_front() {
        let path_count = path_count.entry(cavern).or_insert(0);
        *path_count += 1;

        if cavern.is_small() {
            visited.insert(cavern);
        }
        if cavern != Cavern::End {
            for next in mapping.get(&cavern).unwrap() {
                if *next != Cavern::Start {
                    /* Allow one, and only one, revisit (visited only contains small caves) */
                    if visited.contains(next) {
                        if !has_revisited {
                            visit_queue.push_back((*next, visited.clone(), true));
                        }
                    } else {
                        visit_queue.push_back((*next, visited.clone(), has_revisited));
                    }
                }
            }
        }
    }

    *path_count.get(&Cavern::End).unwrap()
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve_p1(input));
    println!("{}", solve_p2(input));
}

#[test]
fn test_simple_p1() {
    let input = include_str!("input-simple");
    assert_eq!(solve_p1(input), 10);
}

#[test]
fn test_simple_slightly_larger_p1() {
    let input = include_str!("input-simple-slightly-larger");
    assert_eq!(solve_p1(input), 19);
}

#[test]
fn test_simple_even_larger_p1() {
    let input = include_str!("input-simple-even-larger");
    assert_eq!(solve_p1(input), 226);
}

#[test]
fn test_simple_p2() {
    let input = include_str!("input-simple");
    assert_eq!(solve_p2(input), 36);
}

#[test]
fn test_simple_slightly_larger_p2() {
    let input = include_str!("input-simple-slightly-larger");
    assert_eq!(solve_p2(input), 103);
}

#[test]
fn test_simple_even_larger_p2() {
    let input = include_str!("input-simple-even-larger");
    assert_eq!(solve_p2(input), 3509);
}
