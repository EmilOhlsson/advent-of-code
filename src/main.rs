use std::fs::File;
use std::io::{BufRead, BufReader};

fn contains_abba(addr_str: &str) -> bool {
    let addr: Vec<u8> = addr_str.bytes().collect();
    if addr.len() < 4 {
        return false; 
    }

    for i in 3..addr.len() {
        if addr[i - 2] == addr[i - 1]  && addr[i - 3] == addr[i] && addr[i] != addr[i - 1] {
            return true;
        }
    }
    return false;
}

fn is_bracket(ch: char) -> bool {
    match ch {
        '[' | ']' => true,
        _ => false,
    }
}

fn tls_capable(addr: String) -> bool {
    let mut abba_in_addr = false;
    let mut abba_in_hyper = false;
    let parts: Vec<&str> = addr.split(is_bracket).collect();
    for (i, addr_part) in parts.iter().enumerate() {
        if i % 2 == 0 {
            abba_in_addr = abba_in_addr || contains_abba(addr_part);
        } else {
            abba_in_hyper = abba_in_hyper || contains_abba(addr_part);
        }
    }
    abba_in_addr && !abba_in_hyper
}

#[test]
fn test_abba() {
    assert_eq!(contains_abba("aaaa"), false);
    assert_eq!(contains_abba("abba"), true);
    assert_eq!(contains_abba("abcd"), false);
    assert_eq!(contains_abba("abcdefghijklmnopqrstuv"), false);
    assert_eq!(contains_abba("abcdefghijklmnopabba"), true);
    assert_eq!(contains_abba("abbabcdefghijklmnopab"), true);
}

#[test]
fn test_input() {
    assert_eq!(tls_capable(String::from("abba[mnop]qrst")), true);
    assert_eq!(tls_capable(String::from("abcd[bddb]xyyx")), false);
    assert_eq!(tls_capable(String::from("aaaa[qwer]tyui")), false);
    assert_eq!(tls_capable(String::from("ioxxoj[asdfgh]zxcvbn")), true);
}

fn main() {
    let mut capable: usize = 0;
    let reader = match File::open("input.txt") {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("Unable to open input.txt: {}", e),
    };

    for (_, line)  in reader.lines().enumerate() {
        if tls_capable(line.unwrap()) {
            capable += 1;
        }
    }
    println!("{} TLS capable IPv7 addresses detected", capable);
}
