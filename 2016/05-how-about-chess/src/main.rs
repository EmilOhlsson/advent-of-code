extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

fn valid_hash(hash: &[u8; 16]) -> bool {
    hash[0] as usize + hash[1] as usize + (hash[2] >> 4) as usize == 0
}

fn get_code_v1(door: String) -> Option<String> {
    let mut hasher = Md5::new();
    let key = door.as_bytes();
    let mut code = String::new();
    for i in 0..std::u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);
        if valid_hash(&output) {
            println!("Found digit: {}", i);
            code.push_str(format!("{:x}", output[2] & 0xf).as_str());
            if code.len() >= 8 {
                return Some(code);
            }
        }
        hasher.reset();
    }
    None
}

fn get_code_v2(door: String) -> Option<String> {
    let mut hasher = Md5::new();
    let key = door.as_bytes();
    let mut code: [Option<String>; 8] = Default::default();

    for i in 0..std::u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut hash = [0; 16];
        hasher.result(&mut hash);
        if valid_hash(&hash) {
            let pos = (hash[2] & 0xfu8) as usize;
            let dig = (hash[3] >> 4u8) as usize;
            if pos < code.len() && code[pos].is_none() {
                println!("Found digit: {}:{:x} - {:?}", pos, dig, hash);
                code[pos] = Some(format!("{:x}", dig));
                if code.iter().find(|e| e.is_none()).is_none() {
                    return Some(code.iter().map(|e| e.clone().unwrap()).collect::<String>());
                }
            }
        }
        hasher.reset();
    }
    None
}

#[test]
fn test_v1() {
    assert_eq!(
        get_code_v1(String::from("abc")),
        Some(String::from("18f47a30"))
    );
}

#[test]
fn test_v2() {
    assert_eq!(
        get_code_v2(String::from("abc")),
        Some(String::from("05ace8e3"))
    );
}

fn main() {
    println!("{}", get_code_v1(String::from("wtnhxymk")).unwrap());
    println!("{}", get_code_v2(String::from("wtnhxymk")).unwrap());
}
