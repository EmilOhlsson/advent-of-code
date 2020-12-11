type Pos = (i32, i32);
type Seating = std::collections::HashMap<Pos, char>;

fn build_seating(input: &str) -> Seating {
    let mut seating = Seating::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            seating.insert((row as i32, col as i32), ch);
        }
    }
    seating
}

fn count_neighbors_v1((row, col): &Pos, seating: &Seating) -> u32 {
    let mut count = 0;
    for (dr, dc) in &[
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
    ] {
        if let Some(&ch) = seating.get(&(row + dr, col + dc)) {
            count += (ch == '#') as u32;
        }
    }
    count
}

fn count_neighbors_v2((row, col): &Pos, seating: &Seating) -> u32 {
    let mut count = 0;
    for (dr, dc) in &[
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
    ] {
        let mut ring = 1;
        while let Some(&ch) = seating.get(&(row + dr * ring, col + dc * ring)) {
            if ch != '.' {
                count += (ch == '#') as u32;
                break;
            } else {
                ring += 1;
            }
        }
    }
    count
}

fn solve(input: &str, tolerance: u32, counter: &dyn Fn(&Pos, &Seating) -> u32) -> u32 {
    let mut seating = build_seating(input);

    loop {
        let mut seating_new = seating.clone();
        for (pos, ch) in seating_new.iter_mut() {
            *ch = match *seating.get(&pos).unwrap() {
                '#' => {
                    if counter(pos, &seating) >= tolerance {
                        'L'
                    } else {
                        '#'
                    }
                }
                'L' => {
                    if counter(pos, &seating) == 0 {
                        '#'
                    } else {
                        'L'
                    }
                }
                '.' => '.',
                _ => panic!(),
            };
        }
        if seating_new == seating {
            return seating.iter().map(|(_, ch)| (*ch == '#') as u32).sum();
        } else {
            seating = seating_new;
        }
    }
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input, 4, &count_neighbors_v1));
    println!("{}", solve(input, 5, &count_neighbors_v2));
}

#[test]
fn test_simple() {
    let input = include_str!("input-simple");
    assert_eq!(solve(input, 4, &count_neighbors_v1), 37);
    assert_eq!(solve(input, 5, &count_neighbors_v2), 26);
}
