#![feature(step_by)]

use std::fmt;
use std::collections::{BinaryHeap, HashSet, HashMap};
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

const FLOORS: isize = 4;
const TYPES: usize = 10;

type Move = [isize; TYPES];

#[derive(Copy, Clone)]
struct State {
    elev: isize,
    floors: [isize; TYPES],
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.elev, self.floors)
    }
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        // TODO could be that this need to be optimized
        self.elev == other.elev && self.floors == other.floors
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.elev.hash(state);
        self.floors.hash(state);
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.distance().cmp(&self.distance())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn distance(&self) -> isize {
        self.floors.iter().fold(0, |sum, floor| sum + FLOORS - floor)
    }

    fn adjacent_states(&self) -> Vec<State> {
        fn valid_move(state: &State, dir: isize, mv: &Move) -> Option<State> {
            let f: Vec<isize> = state.floors.iter().zip(mv.iter()).map(|(a, b)| a + b).collect();
            let e = state.elev + dir;

            // Look for killed chips
            for g in (0..TYPES).step_by(2) {
                for m in (1..TYPES).step_by(2) {
                    if f[m] == f[g] && f[m] != f[m - 1] {
                        return None;
                    }
                }
            }

            Some(State {
                elev: e,
                floors: [f[0], f[1], f[2], f[3], f[4],  f[5],  f[6],  f[7],  f[8],  f[9]],
            })
        }
        fn get_moves(state: &State,
                     dir: isize,
                     offs: usize,
                     filled: isize,
                     moves: &mut Vec<State>,
                     buffer: &mut Move) {
            if filled >= 2 || offs >= buffer.len() {
                return;
            }
            for i in (offs)..(buffer.len()) {
                if state.floors[i] == state.elev {
                    get_moves(state, dir, i + 1, filled, moves, buffer);

                    buffer[i] = dir;
                    if let Some(new_state) = valid_move(&state, dir, &buffer) {
                        moves.push(new_state);
                    }

                    get_moves(state, dir, i + 1, filled + 1, moves, buffer);
                    buffer[i] = 0;
                    return;
                }
            }
        }

        let mut mv = Vec::new();

        if self.elev + 1 <= FLOORS as isize {
            let mut buffer: Move = [0; TYPES];
            get_moves(self, 1, 0, 0, &mut mv, &mut buffer);
        }

        if self.elev - 1 >= 1 as isize {
            let mut buffer: Move = [0; TYPES];
            // No point in moving two down
            get_moves(self, -1, 0, 0, &mut mv, &mut buffer);
        }

        mv
    }

    fn game_tree_search(&self) -> Option<usize>  {
        let mut closed: HashSet<State> = HashSet::new();
        let mut open: BinaryHeap<State> = BinaryHeap::new();
        let mut g_score: HashMap<State, usize> = HashMap::new();

        open.push(self.clone());
        g_score.entry(*self).or_insert(0);

        while let Some(current) = open.pop() {
            if current.distance() == 0 {
                return Some(g_score[&current]);
            }
            closed.insert(current.clone());
            let neighbours: Vec<State> = current.adjacent_states();
            for n in neighbours.iter().filter(|s| !closed.contains(s)) {
                if open.iter().find(|&&x| x == *n) == None {
                    open.push(n.clone());
                }
                let tent_gscore = g_score[&current] + 1;
                let score = g_score.entry(*n).or_insert(std::usize::MAX);
                if *score >= tent_gscore {
                    *score = tent_gscore;
                }
            }
        }

        None
    }
}

fn main() {
    let state = State {
        elev: 1,
        floors: [1, 1, 1, 1, 2, 3, 2, 2, 2, 2],
    };

    if let Some(moves) = state.game_tree_search() {
        println!("Took {} moves to finish", moves);
    } else {
        println!("Unable to find path");
    }
}
