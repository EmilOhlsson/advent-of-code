use std::collections::HashMap;

fn dance(cache: &mut HashMap<Vec<char>, Vec<char>>, moves: &Vec<&str>, mut dancers: Vec<char>) -> Vec<char> {
	if let Some(ds) = cache.get(&dancers) {
		return ds.clone();
	}
	let s0 = dancers.clone();
	for &m in moves {
		let ts = m.chars().collect::<Vec<char>>();
		match ts[0] {
			's' => {
				let mut dancers_new;
				let i = m.trim_left_matches('s').parse::<usize>().unwrap();
				{
					let (a, b) = dancers.split_at(dancers.len() - i);
					dancers_new = b.to_vec();
					dancers_new.append(&mut a.to_vec());
				}
				for (i, &ch) in dancers_new.iter().enumerate() {
					dancers[i] = ch;
				}
			},
			'x' => {
				// swap
				let is = m.trim_left_matches('x').split('/').map(|t| t.parse::<usize>().unwrap()).collect::<Vec<usize>>();
				let tmp = dancers[is[0]];
				dancers[is[0]] = dancers[is[1]];
				dancers[is[1]] = tmp;

			},
			'p' => {
				// Partner
				let i0 = dancers.iter().position(|&c| ts[1] == c).unwrap();
				let i1 = dancers.iter().position(|&c| ts[3] == c).unwrap();
				let tmp = dancers[i0];
				dancers[i0] = dancers[i1];
				dancers[i1] = tmp;
			},
			_ => panic!("What is {}", ts[0]),
		}
	}
	cache.insert(s0, dancers.clone());
	return dancers;
}

fn main() {
	let mut dancers: Vec<char> = "abcdefghijklmnop".chars().collect::<Vec<char>>();
    let input = include_str!("input.txt").trim().split(',').collect::<Vec<&str>>();
    let mut cache = HashMap::new();
    for _ in 0..1_000_000_000 {
    	dancers = dance(&mut cache, &input, dancers);
	}
    println!("{}", dancers.iter().collect::<String>());
}

#[test]
fn test_input() {
	let mut dancers = vec!['a', 'b', 'c', 'd', 'e'];
	let input = vec!["s1", "x3/4", "pe/b"];
	for _ in 0..1_000_000_001 {
		dance(&input, &mut dancers);
	}
	
	assert_eq!(dancers.iter().collect::<String>(), "baedc");
}
