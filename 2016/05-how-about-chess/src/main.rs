extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn get_code(door: String) -> Option<String> {
    let mut hasher = Md5::new();
    let key = door.as_bytes();
    let mut code = String::new();
    for i in 0..std::u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);
        if output[0] as usize + output[1] as usize + (output[2] >> 4) as usize  == 0 {
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

#[test]
fn simple_code() {
    assert_eq!(get_code(String::from("abc")), Some(String::from("18f47a30")));
}

fn main() {
    println!("{}", get_code(String::from("wtnhxymk")).unwrap());
}
