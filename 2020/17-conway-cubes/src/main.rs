use std::collections::HashSet;

type Xyz = (i32, i32, i32);
type Xyzw = (i32, i32, i32, i32);

fn cube_around(pos: Xyz) -> impl Iterator<Item = Xyz> {
    (-1..=1).flat_map(move |dx| {
        (-1..=1).flat_map(move |dy| (-1..=1).map(move |dz| (pos.0 + dx, pos.1 + dy, pos.2 + dz)))
    })
}

fn neighbors_of(pos: Xyz) -> impl Iterator<Item = Xyz> {
    cube_around(pos).filter(move |&p| p != pos)
}

fn solve(input: &str) -> usize {
    let mut active = HashSet::<Xyz>::new();
    let mut consider = HashSet::<Xyz>::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                let pos = (col as i32, row as i32, 0);
                active.insert(pos);
                for neighor in cube_around(pos) {
                    consider.insert(neighor);
                }
            }
        }
    }

    for _turn in 1..=6 {
        let mut active_next = HashSet::<Xyz>::new();
        let mut consider_next = HashSet::<Xyz>::new();

        for candidate in &consider {
            let is_active = active.contains(candidate);
            let count = neighbors_of(*candidate)
                .filter(|neighbor| active.contains(neighbor))
                .count();
            if is_active && count == 2 || count == 3 {
                // Cube becomes/stays active
                active_next.insert(*candidate);
                for p in cube_around(*candidate) {
                    consider_next.insert(p);
                }
            }
        }

        active = active_next;
        consider = consider_next;
    }
    active.len()
}

fn hypercube_around(pos: Xyzw) -> impl Iterator<Item = Xyzw> {
    (-1..=1).flat_map(move |dx| {
        (-1..=1).flat_map(move |dy| {
            (-1..=1).flat_map(move |dz| {
                (-1..=1).map(move |dw| (pos.0 + dx, pos.1 + dy, pos.2 + dz, pos.3 + dw))
            })
        })
    })
}

fn hyperneighbors_of(pos: Xyzw) -> impl Iterator<Item = Xyzw> {
    hypercube_around(pos).filter(move |&p| p != pos)
}

fn solve_p2(input: &str) -> usize {
    let mut active = HashSet::<Xyzw>::new();
    let mut consider = HashSet::<Xyzw>::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                let pos = (col as i32, row as i32, 0, 0);
                active.insert(pos);
                for neighor in hypercube_around(pos) {
                    consider.insert(neighor);
                }
            }
        }
    }

    for _turn in 1..=6 {
        let mut active_next = HashSet::<Xyzw>::new();
        let mut consider_next = HashSet::<Xyzw>::new();

        for candidate in &consider {
            let is_active = active.contains(candidate);
            let count = hyperneighbors_of(*candidate)
                .filter(|neighbor| active.contains(neighbor))
                .count();
            if is_active && count == 2 || count == 3 {
                // Cube becomes/stays active
                active_next.insert(*candidate);
                for p in hypercube_around(*candidate) {
                    consider_next.insert(p);
                }
            }
        }

        active = active_next;
        consider = consider_next;
    }
    active.len()
}
fn main() {
    let input = include_str!("input");
    println!("{}", solve(input));
    println!("{}", solve_p2(input));
}

#[test]
fn test() {
    let input = include_str!("input-simple");
    assert_eq!(solve(input), 112);
}
