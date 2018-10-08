extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

use std::collections::{HashSet, VecDeque};

fn hash(input: &str) -> Vec<char> {
    let mut hasher = Md5::new();
    let mut tmp = input.to_owned();

    for _ in 0..2017 {
        hasher.reset();
        hasher.input_str(&tmp);
        tmp = hasher.result_str();
    }

    tmp.chars().collect()
}

fn find_index(salt: &str) -> usize {
    let mut index = 0;
    let mut key_set: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<(usize, char, String)> = VecDeque::new();

    loop {
        let nib = hash(&format!("{}{}", salt, index));

        for i in 2..nib.len() {
            if nib[i] == nib[i - 1] && nib[i] == nib[i - 2] {
                queue.push_back((index, nib[i], nib.iter().collect()));
                break;
            }
        }

        queue
            .iter()
            .filter(|(i, _, _)| index - i <= 1000 && index > *i)
            .for_each(|(i, n, _)| {
                for j in 4..nib.len() {
                    if nib[j] == *n
                        && nib[j] == nib[j - 1]
                        && nib[j] == nib[j - 2]
                        && nib[j] == nib[j - 3]
                        && nib[j] == nib[j - 4]
                    {
                        key_set.insert(*i);
                        break;
                    }
                }
            });
        if key_set.len() >= 64 {
            let mut keys = key_set.into_iter().collect::<Vec<usize>>();
            keys.sort_unstable();
            return keys[63];
        }

        index += 1;
    }
}

fn main() {
    println!("cuanljph: {}", find_index("cuanljph"));
    // Should be 20606
}
