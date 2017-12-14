fn knot_hash(input_str: &str) -> Vec<usize> {
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
	return codes.chunks(16).map(|ch| ch.iter().fold(0,|a, &v| a ^ v)).collect::<Vec<usize>>();
}

fn fragmentation_count(input_str: &str) -> usize {
    let mut count = 0usize;
    for i in 0..128 {
        let lcode = format!("{}-{}", input_str, i);
        let linehash = knot_hash(&lcode);
        count += linehash.iter().map(|v| v.count_ones() as usize).sum::<usize>();
    }
    return count;
}

fn main() {
    let input = "nbysizxe";
    println!("{}", fragmentation_count(&input));
}

#[test]
fn test_count() {
	assert_eq!(fragmentation_count("flqrgnkx"), 8108);
}
