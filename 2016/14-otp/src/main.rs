extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

fn to_nibbles(itr: Iterator<u8>) -> Vec<u8> {
    let mut res = Vec::new();
    for i in itr {
        res.push(i >> 4);
        res.push(i & 0xfu8);
    }
    res
}

fn nibble_hash(data: &str) {
    let mut hasher = Md5::new();

    while false {
        let mut hash = [0; 16];
        hasher.input(data.as_bytes());
        hasher.result(&hash);
        let nib = to_nibbles(hash.iter());
        hasher.reset();
    }
}

fn find_index(in: &str) -> usize {
    unimplemented!();
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_simple_p1() {
    assert_eq!(find_index("abc"), 123);
}