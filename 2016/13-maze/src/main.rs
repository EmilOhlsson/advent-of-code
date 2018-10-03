#[macro_use]
extern crate coding_challenge_utils;

use coding_challenge_utils::coord::Cartesian;
use coding_challenge_utils::{graph, graph::Vertex};

use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Node {
    num: i32,
    coord: Cartesian,
}

impl graph::Vertex for Node {
    fn distance(&self, other: &Self) -> usize {
        self.coord.manhattan_distance(&other.coord)
    }

    fn neighbors(&self) -> Vec<Rc<Self>> {
        self.coord
            .neigh4()
            .iter()
            .filter_map(|coord| {
                let n = Node::new(self.num, coord.x, coord.y);
                if n.passable() {
                    Some(Rc::new(n))
                } else {
                    None
                }
            }).collect()
    }
}

impl Node {
    fn new(num: i32, x: i32, y: i32) -> Node {
        Node {
            num,
            coord: Cartesian::new(x, y),
        }
    }

    fn passable(&self) -> bool {
        !is_wall(self.num, self.coord.x, self.coord.y) && self.coord.x >= 0 && self.coord.y >= 0
    }
}

fn is_wall(num: i32, x: i32, y: i32) -> bool {
    (x * x + 3 * x + 2 * x * y + y + y * y + num).count_ones() % 2 != 0
}

#[test]
fn test_first() {
    if let Some(path) = graph::search(Rc::new(Node::new(10, 1, 1)), Rc::new(Node::new(10, 7, 4))) {
        for n in &path {
            println!("({:?})", n);
        }
        assert_eq!(path.len() - 1, 11);
    } else {
        panic!("Unable to find path");
    }
}

fn flow(dist: &mut HashMap<Node, usize>, node: &Node) {
    let mut queue: VecDeque<Node> = VecDeque::new();
    let mut visited: HashSet<Node> = HashSet::new();

    queue.push_back(node.clone());
    dist.insert(node.clone(), 0);
    while let Some(n) = queue.pop_front() {
        if !visited.contains(&n) {
            for neigh in n.neighbors().iter().map(Rc::as_ref) {
                let new_dist = dist.get(&n).unwrap() + 1;
                if new_dist <= 50 {
                    queue.push_back(neigh.clone());
                    dist.insert(neigh.clone(), new_dist);
                }
            }
            visited.insert(n);
        }
    }
}

fn main() {
    let num: i32 = 1362;
    let start = Node::new(num, 1, 1);
    let goal = Node::new(num, 31, 39);
    if let Some(path) = graph::search(Rc::new(start), Rc::new(goal)) {
        answer!("{}", path.len() - 1);
    } else {
        panic!("No path found");
    }

    let mut unique: HashMap<Node, usize> = HashMap::new();
    flow(&mut unique, &Node::new(num, 1, 1));
    answer!("{}", unique.len());
}
