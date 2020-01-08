use std::collections::HashSet;

type Board = HashSet<(i32, i32)>;
type RecursiveBoard = HashSet<(i32, i32, i32)>;

fn next_gen(board: &Board, r: i32, c: i32) -> bool {
    let alive = board.contains(&(r, c));
    let mut count = 0;
    for (dr, dc) in &[(-1, 0), (0, -1), (1, 0), (0, 1)] {
        count += board.contains(&(r + dr, c + dc)) as i32;
    }

    if alive && count != 1 {
        false
    } else if !alive && (count == 1 || count == 2) {
        true
    } else {
        alive
    }
}

fn _print_board(board: &Board) {
    for r in 0..5 {
        for c in 0..5 {
            print!("{}", if board.contains(&(r, c)) { '#' } else { '.' });
        }
        println!();
    }
    println!()
}

fn board_to_div(board: &Board) -> u32 {
    let mut result = 0;
    for p in board {
        result |= 1 << (p.0 * 5 + p.1);
    }
    result
}

fn solve(input: &str) -> u32 {
    let mut gol = HashSet::new();
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == '#' {
                gol.insert((r as i32, c as i32));
            }
        }
    }
    let mut generations = vec![board_to_div(&gol)];
    let mut deltas = HashSet::new();
    for _time in 1.. {
        let mut gol_next = HashSet::new();
        for r in 0..5 {
            for c in 0..5 {
                if next_gen(&gol, r, c) {
                    gol_next.insert((r, c));
                }
            }
        }
        let div = board_to_div(&gol_next);
        if !deltas.insert(div) {
            return div;
        }
        generations.push(div);

        gol = gol_next;
    }
    unreachable!()
}

fn _print_recursive_board(board: &RecursiveBoard, level: i32) {
    for r in 0..5 {
        for c in 0..5 {
            print!(
                "{}",
                if board.contains(&(r, c, level)) {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!();
    }
    println!()
}

// Get neighbors of a point
fn neighbors((r, c, l): (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    let mut neighbors = Vec::new();
    for &(dr, dc) in &[(-1, 0), (0, -1), (1, 0), (0, 1)] {
        let r_new = r + dr;
        let c_new = c + dc;
        if r_new < 0 {
            neighbors.push((1, 2, l - 1));
        } else if r_new >= 5 {
            neighbors.push((3, 2, l - 1));
        } else if c_new < 0 {
            neighbors.push((2, 1, l - 1));
        } else if c_new >= 5 {
            neighbors.push((2, 3, l - 1));
        } else if r_new == 2 && c_new == 2 {
            if r == 1 {
                (0..5).for_each(|v| neighbors.push((0, v, l + 1)));
            } else if r == 3 {
                (0..5).for_each(|v| neighbors.push((4, v, l + 1)));
            } else if c == 1 {
                (0..5).for_each(|v| neighbors.push((v, 0, l + 1)));
            } else if c == 3 {
                (0..5).for_each(|v| neighbors.push((v, 4, l + 1)));
            }
        } else {
            neighbors.push((r_new, c_new, l));
        }
    }
    neighbors
}

fn solve_v2(input: &str) -> usize {
    let mut gol = RecursiveBoard::new();
    let mut candidates = RecursiveBoard::new();
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == '#' {
                let bug = (r as i32, c as i32, 0);
                neighbors(bug).iter().for_each(|&n| {
                    candidates.insert(n);
                });
                gol.insert(bug);
            }
        }
    }

    for _ in 0..200 {
        let mut gol_next = RecursiveBoard::new();
        let mut candidates_next = RecursiveBoard::new();

        for &candidate in &candidates {
            let mut n_count = 0;
            for n in neighbors(candidate) {
                n_count += gol.contains(&n) as i32;
            }
            let mut alive = gol.contains(&candidate);
            alive = if alive && n_count != 1 {
                false
            } else if !alive && (n_count == 1 || n_count == 2) {
                true
            } else {
                alive
            };
            if alive {
                gol_next.insert(candidate);
                neighbors(candidate).iter().for_each(|&n| {
                    candidates_next.insert(n);
                });
            }
        }

        gol = gol_next;
        candidates = candidates_next;
    }

    gol.len()
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input));
    println!("{}", solve_v2(input));
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("input-test")), 2129920);
}

#[test]
fn test_regursive_neighbors() {
    assert_eq!(neighbors((3, 3, 0)).len(), 4);
    assert_eq!(neighbors((2, 3, 0)).len(), 8);
    assert_eq!(neighbors((2, 4, 0)).len(), 4);
}
