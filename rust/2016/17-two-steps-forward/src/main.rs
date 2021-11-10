extern crate coding_challenge_utils;
extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

use coding_challenge_utils::coord;
use coding_challenge_utils::graph;

use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Room {
    pos: coord::Cartesian,
    path: String,
}

impl Room {
    fn new(pos: coord::Cartesian, path: String) -> Rc<Room> {
        Rc::new(Room { pos, path })
    }
}

impl graph::Vertex for Room {
    fn neighbors(&self) -> Vec<Rc<Self>> {
        let up = coord::Cartesian::new(0, -1);
        let down = coord::Cartesian::new(0, 1);
        let left = coord::Cartesian::new(-1, 0);
        let right = coord::Cartesian::new(1, 0);

        let mut neighs = Vec::new();
        let opened = hash(&self.path);
        if opened[0] && self.pos.y > 0 {
            neighs.push(Room::new(&self.pos + &up, format!("{}{}", self.path, 'U')));
        }
        if opened[1] && self.pos.y < 3 {
            neighs.push(Room::new(
                &self.pos + &down,
                format!("{}{}", self.path, 'D'),
            ));
        }
        if opened[2] && self.pos.x > 0 {
            neighs.push(Room::new(
                &self.pos + &left,
                format!("{}{}", self.path, 'L'),
            ));
        }
        if opened[3] && self.pos.x < 3 {
            neighs.push(Room::new(
                &self.pos + &right,
                format!("{}{}", self.path, 'R'),
            ));
        }

        neighs
    }

    fn distance(&self, other: &Self) -> usize {
        self.pos.manhattan_distance(&other.pos)
    }
}

fn is_open(ch: char) -> bool {
    !(ch.is_numeric() || ch == 'a')
}

fn hash(s: &str) -> Vec<bool> {
    let mut hasher = Md5::new();
    hasher.input_str(s);
    hasher.result_str().chars().take(4).map(is_open).collect()
}

fn solve_p1(key: &str) -> Option<String> {
    let start = Room::new(coord::Cartesian::new(0, 0), key.to_owned());
    let goal = Room::new(coord::Cartesian::new(3, 3), String::new());
    if let Some(path) = graph::astar_search::<Room>(start, goal) {
        let mut p = path[0].path.to_string();
        return Some(p.split_off(key.len()));
    }
    None
}

fn solve_p2(key: &str) -> Option<usize> {
    let start = Room::new(coord::Cartesian::new(0, 0), key.to_owned());
    let goal = Room::new(coord::Cartesian::new(3, 3), String::new());
    let paths = graph::bfs_search_all::<Room>(start, goal);
    paths.iter().max_by_key(|v| v.len()).map(|v| v.len() - 1)
}

fn main() {
    println!("{}", solve_p1("mmsxrhfx").unwrap());
    println!("{}", solve_p2("mmsxrhfx").unwrap());
}

// Test section ////////////////////////////////////////////////////////////////

#[test]
fn test_hashing() {
    assert_eq!(hash("hijkl"), vec![true, true, true, false]);
    assert_eq!(hash("hijklD"), vec![true, false, true, true]);
}

#[test]
fn test_neigh() {
    let room0 = Room::new(coord::Cartesian::new(0, 0), "hijkl".to_string());
    let room1 = Room::new(coord::Cartesian::new(0, 1), "hijklD".to_string());
    let room2 = Room::new(coord::Cartesian::new(0, 0), "hijklDU".to_string());

    use coding_challenge_utils::graph::Vertex;
    assert_eq!(
        room0.neighbors(),
        vec![Room::new(coord::Cartesian::new(0, 1), "hijklD".to_string())]
    );
    assert_eq!(
        room1.neighbors(),
        vec![
            Room::new(coord::Cartesian::new(0, 0), "hijklDU".to_string()),
            Room::new(coord::Cartesian::new(1, 1), "hijklDR".to_string()),
        ]
    );
    assert_eq!(
        room2.neighbors(),
        vec![Room::new(
            coord::Cartesian::new(1, 0),
            "hijklDUR".to_string()
        )]
    );
}

#[test]
fn test_simple() {
    assert_eq!(solve_p1("ihgpwlah"), Some("DDRRRD".to_string()));
    assert_eq!(solve_p1("kglvqrro"), Some("DDUDRLRRUDRD".to_string()));
    assert_eq!(
        solve_p1("ulqzkmiv"),
        Some("DRURDRUDDLLDLUURRDULRLDUUDDDRR".to_string())
    );
    assert_eq!(solve_p2("ihgpwlah"), Some(370));
    assert_eq!(solve_p2("kglvqrro"), Some(492));
    assert_eq!(solve_p2("ulqzkmiv"), Some(830));
}
