use itertools::izip;

fn has_three(password: &str) -> bool {
    for (a, b, c) in izip!(
        password.bytes(),
        password.bytes().skip(1),
        password.bytes().skip(2)
    ) {
        if a + 2 == c && b + 1 == c {
            return true;
        }
    }
    false
}

fn has_forbidden(password: &str) -> bool {
    password.chars().any(|ch| match ch {
        'i' | 'o' | 'l' => true,
        _ => false,
    })
}

fn has_pairs(password: &str) -> bool {
    let mut pairs = std::collections::HashSet::new();
    for (a, b) in izip!(password.bytes(), password.bytes().skip(1)) {
        if a == b {
            pairs.insert(a);
        }
    }
    pairs.len() >= 2
}

fn next_word(input: &str) -> String {
    input
        .chars()
        .rev()
        .fold((1u8, String::new()), |(carry, mut pass), ch| {
            let ord = ch as u8 + carry;
            let (carry, ord_wrapped) = if ord > b'z' {
                (1, ord - b'z' + b'a' - 1)
            } else {
                (0, ord)
            };
            pass.insert(0, ord_wrapped as char);
            (carry, pass)
        })
        .1
}

fn next_passwd(input: &str) -> String {
    let mut prev = input.to_string();
    loop {
        let candidate = next_word(&prev);
        if has_pairs(&candidate) && has_three(&candidate) && !has_forbidden(&candidate) {
            return candidate;
        }
        prev = candidate;
    }
}

fn main() {
    let input = "hepxcrrq";
    println!("{}", next_passwd(input));
    println!("{}", next_passwd(&next_passwd(input)));
}

#[test]
fn test() {
    assert_eq!(next_word("abc"), "abd");
    assert_eq!(next_word("xyz"), "xza");
    assert_eq!(next_passwd("abcdefgh"), "abcdffaa");
    assert_eq!(next_passwd("ghijklmn"), "ghjaabcc");
}
