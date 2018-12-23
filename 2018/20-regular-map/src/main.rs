use coding_challenge_utils::coord::Cartesian as C2d;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

type Maze = HashMap<C2d, Sq>;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Sq {
    Room,
    Door,
    Wall,
}

fn build(
    maze: &mut Maze,
    input: &str,
    i: usize,
    mut pos: C2d,
) -> ((i32, i32), (i32, i32)) {
    let mut prev = Vec::new();
    let mut xr = (0i32, 0i32);
    let mut yr = (0i32, 0i32);
    for (_j, ch) in input.chars().enumerate().skip(i) {
        match ch {
            '^' | '$' => (),
            'N' => {
                pos += C2d::new(0, -1);
                maze.insert(pos, Sq::Door);
                pos += C2d::new(0, -1);
                maze.insert(pos, Sq::Room);
                yr.0 = min(yr.0, pos.y);
            }
            'E' => {
                pos += C2d::new(1, 0);
                maze.insert(pos, Sq::Door);
                pos += C2d::new(1, 0);
                maze.insert(pos, Sq::Room);
                xr.1 = max(xr.1, pos.x);
            }
            'W' => {
                pos += C2d::new(-1, 0);
                maze.insert(pos, Sq::Door);
                pos += C2d::new(-1, 0);
                maze.insert(pos, Sq::Room);
                xr.0 = min(xr.0, pos.x);
            }
            'S' => {
                pos += C2d::new(0, 1);
                maze.insert(pos, Sq::Door);
                pos += C2d::new(0, 1);
                maze.insert(pos, Sq::Room);
                yr.1 = max(yr.1, pos.y);
            }
            '(' => {
                prev.push(pos);
            }
            '|' => {
                pos = *prev.last().unwrap();
            }
            ')' => {
                pos = prev.pop().unwrap();
            }
            _ => panic!("Unhandled {:?}", ch),
        }
    }
    (xr, yr)
}

fn dijksra(maze: &Maze) -> HashMap<C2d, usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();

    queue.push_back(C2d::new(0, 0));
    visited.insert(C2d::new(0, 0));
    distances.insert(C2d::new(0, 0), 0);
    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();
        for neigh in current.neigh4() {
            let ns = maze.get(&neigh);
            if ns == Some(&Sq::Door) {
                let diff = C2d::new(neigh.x - current.x, neigh.y - current.y);
                let new = neigh + diff;
                if visited.insert(new) {
                    let dist = distances.get(&current).unwrap();
                    distances.insert(new, dist + 1);
                    queue.push_back(new);
                }
            }
        }
    }

    distances
}

fn solve(input: &str) -> (usize, usize) {
    let mut maze: Maze = HashMap::new();
    maze.insert(C2d::new(0, 0), Sq::Room);
    let (xr, yr) = build(&mut maze, input, 0, C2d::new(0, 0));

    for y in (yr.0 - 1)..=(yr.1 + 1) {
        for x in (xr.0 - 1)..=(xr.1 + 1) {
            let e = maze.entry(C2d::new(x, y)).or_insert(Sq::Wall);
            print!(
                "{}",
                if (x, y) == (0, 0) {
                    'x'
                } else {
                    match *e {
                        Sq::Room => '.',
                        Sq::Door => '+',
                        Sq::Wall => '#',
                    }
                }
            );
        }
        println!();
    }
    println!();

    let dj = dijksra(&maze);
    for y in (yr.0 - 1)..=(yr.1 + 1) {
        for x in (xr.0 - 1)..=(xr.1 + 1) {
            if let Some(d) = dj.get(&C2d::new(x, y)) {
                print!("{:02} ", d);
            } else {
                print!("-- ");
            }
        }
        println!();
    }
    println!();

    (*dj.values().max().unwrap(), dj.values().filter(|v| *v >= &1000).count())
}

fn main() {
    let input = include_str!("input.txt").trim();
    println!("{:?}", solve(input));
}

#[test]
fn test_trivial0() {
    let re = "^NES$";
    assert_eq!(solve(re), 3);
}

//#[test]
//fn test_trivial1() {
//    let re = "^NN(E(W|N)|W)N$";
//    assert_eq!(solve(re), 3);
//}

#[test]
fn test_sample() {
    let re = "^ENWWW(NEEE|SSE(EE|N))$";
    assert_eq!(solve(re), 10);
}

#[test]
fn test0() {
    let re = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
    assert_eq!(solve(re), 18);
}
#[test]
fn test1() {
    let re = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
    assert_eq!(solve(re), 23);
}

#[test]
fn test2() {
    let re = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
    assert_eq!(solve(re), 31);
}
