use regex::Regex;

fn solve_p1(input: &str) -> i64 {
    let re = Regex::new(r"^(\w)(\d+)$").unwrap();
    let mut pos = (0i64, 0i64); // y positive north, x positive east
    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut dir = 0;
    for capture in input.lines().map(|line| re.captures(line).unwrap()) {
        let cmd = &capture[1];
        let val = capture[2].parse::<i64>().unwrap();
        match cmd {
            "N" => pos = (pos.0, pos.1 + val),
            "S" => pos = (pos.0, pos.1 - val),
            "E" => pos = (pos.0 + val, pos.1),
            "W" => pos = (pos.0 - val, pos.1),
            "F" => pos = (pos.0 + val * dirs[dir].0, pos.1 + val * dirs[dir].1),
            "L" => {
                let new_dir = dir as i64 + val / 90;
                dir = (((new_dir % 4) + 4) % 4) as usize;
            }
            "R" => {
                let new_dir = dir as i64 - val / 90;
                dir = (((new_dir % 4) + 4) % 4) as usize;
            }

            _ => panic!("Unhandled command {}", cmd),
        }
    }

    pos.0.abs() + pos.1.abs()
}

fn solve_p2(input: &str) -> i64 {
    let re = Regex::new(r"^(\w)(\d+)$").unwrap();
    let mut boat = (0i64, 0i64); // y positive north, x positive east
    let mut wayp = (10i64, 1i64); // waypoint position relative to boat
    let rot_x = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    let rot_y = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for capture in input.lines().map(|line| re.captures(line).unwrap()) {
        let cmd = &capture[1];
        let val = capture[2].parse::<i64>().unwrap();
        match cmd {
            "N" => wayp = (wayp.0, wayp.1 + val),
            "S" => wayp = (wayp.0, wayp.1 - val),
            "E" => wayp = (wayp.0 + val, wayp.1),
            "W" => wayp = (wayp.0 - val, wayp.1),
            "F" => boat = (boat.0 + val * wayp.0, boat.1 + val * wayp.1),
            "L" => {
                let rot = val / 90;
                let i = (((rot % 4) + 4) % 4) as usize;
                wayp = (
                    wayp.0 * rot_x[i].0 + wayp.1 * rot_x[i].1,
                    wayp.0 * rot_y[i].0 + wayp.1 * rot_y[i].1,
                );
            }
            "R" => {
                let rot = -val / 90;
                let i = (((rot % 4) + 4) % 4) as usize;
                wayp = (
                    wayp.0 * rot_x[i].0 + wayp.1 * rot_x[i].1,
                    wayp.0 * rot_y[i].0 + wayp.1 * rot_y[i].1,
                );
            }

            _ => panic!("Unhandled command {}", cmd),
        }
    }

    boat.0.abs() + boat.1.abs()
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve_p1(input));
    println!("{}", solve_p2(input));
}

#[test]
fn test() {
    let input = include_str!("input-simple");
    assert_eq!(solve_p1(input), 25);
    assert_eq!(solve_p2(input), 286);
}
