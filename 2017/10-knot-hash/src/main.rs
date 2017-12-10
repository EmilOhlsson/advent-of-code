
fn knot_hash(input: Vec<usize>, length: usize) -> Vec<usize> {
	let mut skip = 0;
	let mut start = 0;
	let mut codes = (0..length).collect::<Vec<usize>>();
	for l in input {
		let end = start + l;
		let mut slice = codes.iter().cycle().skip(start).take(l).cloned().collect::<Vec<usize>>();

		slice.reverse();
		for (i, v) in (start..end).enumerate() {
			codes[v % length] = slice[i];
		}

		start = (start + l + skip) % length;
		skip += 1;
	}
	codes
}

fn main() {
	let input = include_str!("input.txt").trim().split(',')
		.map(|t| t.parse::<usize>().unwrap())
		.collect::<Vec<usize>>();
	let hash = knot_hash(input, 256);
    println!("{}", hash[0] * hash[1]);
}

#[test]
fn test_knot_hash() {
	assert_eq!(knot_hash(vec![3, 4, 1, 5], 5), vec![3,4,2,1,0]);
}