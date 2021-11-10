use std::collections::HashSet;

fn knot_hash(input_str: &str) -> Vec<u8> {
	let mut input = input_str.as_bytes().iter().map(|&b| b as usize).collect::<Vec<usize>>();
	let length = 256;
	let mut skip = 0;
	let mut start = 0;
	let mut codes = (0..length).collect::<Vec<usize>>();
	input.append(&mut vec![17, 31, 73, 47, 23]);
	for _ in 0..64 {
		for &l in &input {
			let end = start + l;
			let mut slice = codes.iter().cycle().skip(start as usize).take(l as usize).cloned().collect::<Vec<usize>>();

			slice.reverse();
			for (i, v) in (start..end).enumerate() {
				codes[v % length] = slice[i];
			}

			start = (start + l + skip) % length;
			skip += 1;
		}
	}
	return codes.chunks(16).map(|ch| ch.iter().fold(0,|a, &v| a as u8 ^ v as u8)).collect::<Vec<u8>>();
}

fn get_rc(input: &Vec<Vec<bool>>, r: isize, c: isize) -> bool {
    if let Some(ref line) = input.get(r as usize) {
        if let Some(&active) = line.get(c as usize) {
            return active;
        }
    }
    return false;
}

fn visit(visited: &mut HashSet<(isize, isize)>, input: &Vec<Vec<bool>>, r: isize, c: isize) {
    if visited.contains(&(r, c)) { return; }

    visited.insert((r,c));
    if get_rc(input, r - 1, c) {visit(visited, input, r - 1, c)}
    if get_rc(input, r, c - 1) {visit(visited, input, r, c - 1)}
    if get_rc(input, r + 1, c) {visit(visited, input, r + 1, c)}
    if get_rc(input, r, c + 1) {visit(visited, input, r, c + 1)}
}

fn count_groups(input: &Vec<Vec<bool>>) -> usize {
    let mut groups = 0;
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    for r in 0..128isize {
        for c in 0..128isize {
            if !input[r as usize][c as usize] || visited.contains(&(r, c)) { continue; }
            groups += 1;
            visit(&mut visited, input, r, c);
        }
    }

    return groups;
}

fn group_count(input_str: &str) -> usize {
    let mut layout: Vec<Vec<bool>> = Vec::new();
    for i in 0..128 {
        let lcode = format!("{}-{}", input_str, i);
        let linehash = knot_hash(&lcode);
        let mut line: Vec<bool> = Vec::new();
        linehash.iter().for_each(|v| {
            line.append(&mut (0..8).map(|j| v & (1 << (8 - j - 1)) != 0).collect::<Vec<bool>>());
        });
        layout.push(line);
    }

    for l in layout.iter() {
        println!("{}", l.iter().map(|&x| if x { '#' } else { ' ' }).collect::<String>());
    }
    return count_groups(&layout);
}

fn main() {
    let input = "nbysizxe";
    println!("{}", group_count(&input));
}

#[test]
fn test_count() {
    println!();
	assert_eq!(group_count("flqrgnkx"), 1242);
}
