
fn knot_hash(input_str: &str) -> String {
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
	let dense = codes.chunks(16).map(|ch| ch.iter().fold(0,|a, &v| a ^ v)).collect::<Vec<usize>>();
	return dense.iter().map(|v| format!("{:02x}", v)).collect::<String>();
}

fn main() {
	let input = include_str!("input.txt").trim();
	let hash = knot_hash(input);
    println!("{}", hash);
}

#[test]
fn test_knot_hash() {
	assert_eq!(knot_hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
	assert_eq!(knot_hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
	assert_eq!(knot_hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
	assert_eq!(knot_hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
}