extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

use std::collections::VecDeque;

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
    let mut keys: Vec<(usize, String, usize, String)> = Vec::new();
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
            .for_each(|(i, n, s)| {
                for j in 4..nib.len() {
                    if nib[j] == *n
                        && nib[j] == nib[j - 1]
                        && nib[j] == nib[j - 2]
                        && nib[j] == nib[j - 3]
                        && nib[j] == nib[j - 4]
                    {
                        keys.push((*i, nib.iter().collect(), index, s.clone()));
                        break;
                    }
                }
            });
        if keys.len() >= 64 {
            keys.sort_unstable();
            keys.iter().for_each(|(k, s, kn, sn)| println!("{}:{}, {}:{}", k, s, kn, sn));
            return keys[63 + 5].0; // There are 5 false finds...
        }

        index += 1;
    }
}

fn main() {
    println!("cuanljph: {}", find_index("cuanljph"));
    // Should be 20606
}
