#![feature(step_by)]

use std::fmt;
use std::collections::VecDeque;

const FLOORS: usize = 4;
const TYPES: usize = 10;

type Move = [isize; TYPES];

struct State {
    elev: isize,
    floors: [isize; TYPES],
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.elev, self.floors)
    }
}

impl State {
    fn done(&self) -> bool {
        self.floors.iter().filter(|&l| *l == FLOORS as isize).count() == TYPES
    }

    fn next(&self) -> Vec<State> {
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
        fn add_moves(state: &State,
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
                    add_moves(state, dir, i + 1, filled, moves, buffer);

                    buffer[i] = dir;
                    if let Some(new_state) = valid_move(&state, dir, &buffer) {
                        moves.push(new_state);
                    }

                    add_moves(state, dir, i + 1, filled + 1, moves, buffer);
                    buffer[i] = 0;
                    return;
                }
            }
        }

        let mut mv = Vec::new();

        if self.elev + 1 <= FLOORS as isize {
            let mut buffer: Move = [0; TYPES];
            add_moves(self, 1, 0, 0, &mut mv, &mut buffer);
        }

        if self.elev - 1 >= 1 as isize {
            let mut buffer: Move = [0; TYPES];
            add_moves(self, -1, 0, 0, &mut mv, &mut buffer);
        }

        mv
    }

    fn game_tree_search(&self) -> usize  {
        let mut queue: VecDeque<(State, usize)> = VecDeque::new();

        // Generate States
        for s in self.next() {
            queue.push_back((s, 1));
        }

        while let Some((state, depth)) = queue.pop_front() {
            for s in state.next() {
                if s.done() {
                    return depth + 1;
                }
                queue.push_back((s, depth + 1));
            }
        }

        panic!("Unable to find finishing move");
    }
}

fn main() {
    let state = State {
        elev: 1,
        floors: [1, 1, 1, 1, 2, 3, 2, 2, 2, 2],
    };

    let moves = state.game_tree_search();
    println!("Took {} moves to finish", moves);
}
/* strontium, plutonium, thulium, ruthenium curium
 *
 * [1, 1, 1, 1, 2, 3, 2, 2, 2, 2] 
 */
