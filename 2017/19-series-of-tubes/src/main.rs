#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
	Up, Down, Left, Right
}

fn get_string(input: &str) -> usize {
	let mut matrix: Vec<Vec<char>> = Vec::new();
	for line in input.lines() {
		let mut l: Vec<char> = Vec::new();
		for ch in line.chars() {
			l.push(ch);
		}
		matrix.push(l);
	}

	let mut dir = Direction::Down;
	let mut r = 0;
	let mut c = 0;

	// Find entry
	for i in 0.. {
		if matrix[0][i] == '|' {
			c = i;
			break;
		}
	}

	let mut steps = 0;
	loop {
		steps += 1;
		let (rn, cn) = match dir {
			Direction::Up => (r - 1, c),
			Direction::Down => (r + 1, c),
			Direction::Left => (r, c - 1),
			Direction::Right => (r, c + 1),
		};
		r = rn;
		c = cn;
		match matrix[r][c] {
			'-'|'|' => (),
			'+' => {
				if dir == Direction::Up || dir == Direction::Down {
					if *matrix[r].get(c - 1).unwrap_or(&' ') ==  ' ' {
						dir = Direction::Right;
					} else {
						dir = Direction::Left;
					}
				} else {
					if let Some(ref row) = matrix.get(r - 1) {
						if row[c] == ' ' {
							dir = Direction::Down;
						}
					} else {
						dir = Direction::Down;
					}
					if let Some(ref row) = matrix.get(r + 1) {
						if row[c] == ' ' {
							dir = Direction::Up;
						}
					} else {
						dir = Direction::Up;
					}
				}
			}
			' ' => break,
			_ => (),
		}
	}

	return steps;
}

fn main() {
	let input = include_str!("input.txt");
	println!("{}", get_string(input));
}

#[test]
fn test_get_string() {
	let input = include_str!("input-simple.txt");
	assert_eq!(get_string(input), 38);
}