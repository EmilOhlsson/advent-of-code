fn contains_abba(addr_str: &str) -> bool {
    let addr: Vec<u8> = addr_str.bytes().collect();
    if addr.len() < 4 {
        return false;
    }

    for i in 3..addr.len() {
        if addr[i - 2] == addr[i - 1] && addr[i - 3] == addr[i] && addr[i] != addr[i - 1] {
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

fn contains_bab(addr: &str, a: u8, b: u8) -> bool {
    let bytes = addr.as_bytes();
    for i in 2..bytes.len() {
        if bytes[i - 2] == bytes[i] && bytes[i] == b && bytes[i - 1] == a {
            return true;
        }
    }
    false
}

fn contains_aba(start: usize, addr: &str) -> Option<(usize, u8, u8)> {
    let bytes = addr.as_bytes();
    for i in (start + 2)..bytes.len() {
        if bytes[i - 2] == bytes[i] {
            return Some((i - 2, bytes[i - 2], bytes[i - 1]));
        }
    }
    None
}

fn ssl_capable(addr_: &str) -> bool {
    let addr = addr_.to_owned();
    let (sup, hyp): (Vec<(usize, &str)>, Vec<(usize, &str)>) = addr
        .split(is_bracket)
        .enumerate()
        .partition(|(i, _)| i % 2 == 0);
    for s in sup.iter().map(|(_, s)| s) {
        let mut i_next = 0;
        while let Some((i, a, b)) = contains_aba(i_next, &s) {
            if hyp
                .iter()
                .map(|(_, s)| s)
                .any(|s| contains_bab(&s, a, b))
            {
                return true;
            } else {
                i_next = i + 1;
            }
        }
    }

    false
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
fn test_ssl() {
    assert_eq!(ssl_capable("aba[bab]xyz"), true);
    assert_eq!(ssl_capable("xyx[xyx]xyx"), false);
    assert_eq!(ssl_capable("aaa[kek]eke"), true);
    assert_eq!(ssl_capable("zazbz[bzb]cdb"), true);
}

#[test]
fn test_tls() {
    assert_eq!(tls_capable(String::from("abba[mnop]qrst")), true);
    assert_eq!(tls_capable(String::from("abcd[bddb]xyyx")), false);
    assert_eq!(tls_capable(String::from("aaaa[qwer]tyui")), false);
    assert_eq!(tls_capable(String::from("ioxxoj[asdfgh]zxcvbn")), true);
}

fn main() {
    let mut tls_cap: usize = 0;
    let mut ssl_cap: usize = 0;

    let lines = include_str!("../input.txt");

    for line in lines.lines() {
        if ssl_capable(&line) {
            ssl_cap += 1;
        }
        if tls_capable(line.to_owned()) {
            tls_cap += 1;
        }
    }
    println!("{} TLS capable IPv7 addresses detected", tls_cap);
    println!("{} SSL capable IPv7 addresses detected", ssl_cap);
}
