#![feature(step_by)]

use std::fmt;
use std::collections::{BinaryHeap, HashSet, HashMap};
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

const FLOORS: isize = 4;
const TYPES: usize = 14;

type Move = [isize; TYPES];

#[derive(Copy, Clone)]
struct State {
    elev: isize,
    floors: [isize; TYPES],
}

struct ScoredState {
    state: State,
    score: usize,
}

impl ScoredState {
    fn new(state: State, score: usize) -> ScoredState {
        ScoredState {
            state: state,
            score: score,
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.elev, self.floors)
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.elev.hash(state);
        self.floors.hash(state);
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        // TODO could be that this need to be optimized
        self.elev == other.elev && self.floors == other.floors
    }
}

impl Eq for ScoredState {}

impl PartialEq for ScoredState {
    fn eq(&self, other: &ScoredState) -> bool {
        self.state == other.state
    }
}

impl Ord for ScoredState {
    fn cmp(&self, other: &ScoredState) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for ScoredState {
    fn partial_cmp(&self, other: &ScoredState) -> Option<Ordering> {
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
                floors: [f[0], f[1], f[2], f[3], f[4],  f[5],  f[6],  f[7],  f[8],  f[9], f[10], f[11], f[12], f[13]],
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
            get_moves(self, -1, 1, 0, &mut mv, &mut buffer);
        }

        mv
    }

    fn game_tree_search(&self) -> Option<usize>  {
        let mut open: BinaryHeap<ScoredState> = BinaryHeap::new();
        let mut closed: HashSet<State> = HashSet::new();

        /* g_score, cost of getting from start to that node */
        let mut g_score: HashMap<State, usize> = HashMap::new();

        /* f_score, the cost of getting from start to finish by passing that node */
        let mut f_score: HashMap<State, usize> = HashMap::new();

        open.push(ScoredState::new(self.clone(), 100));
        g_score.entry(*self).or_insert(0);
        f_score.entry(*self).or_insert(100);

        /* Debugging */
        let mut temp_level = 1usize;
        let mut temp_track = 0usize;

        while let Some(ScoredState{state: current, ..}) = open.pop() {
            /* Length is checked above */
            if current.distance() == 0 {
                return Some(g_score[&current]);
            }

            temp_track += 1;
            if temp_track >= temp_level {
                temp_level *= 10;
                println!("{} entries in open, and current passing score is {}, and node score is {}, distance {}", 
                         open.len(), f_score[&current], g_score[&current], current.distance());
            }

            /* No need to revisit */
            closed.insert(current.clone());
            let neighbours: Vec<State> = current.adjacent_states();
            for n in neighbours.iter().filter(|s| !closed.contains(s)) {
                let tent_gscore = g_score[&current] + 1;
                let tent_fscore = tent_gscore + n.distance() as usize;

                open.push(ScoredState::new(n.clone(), tent_fscore));
                if tent_gscore < *g_score.entry(*n).or_insert(std::usize::MAX) {
                    g_score.insert(*n, tent_gscore);
                    f_score.insert(*n, tent_gscore + n.distance() as usize);
                }
            }
        }

        None
    }
}

fn main() {
    let state = State {
        elev: 1,
        floors: [1, 1, 1, 1, 2, 3, 2, 2, 2, 2, 1, 1, 1, 1],
    };

    if let Some(moves) = state.game_tree_search() {
        println!("Took {} moves to finish", moves);
    } else {
        println!("Unable to find path");
    }
}
