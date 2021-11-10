fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn nice_v1(s: &&str) -> bool {
    if s.is_empty() {
        return false;
    }
    let chars = s.chars().collect::<Vec<char>>();
    let mut vowels = is_vowel(chars[0]) as u32;
    let mut doubles = 0;
    for i in 1..chars.len() {
        doubles += (chars[i] == chars[i - 1]) as u32;
        vowels += is_vowel(chars[i]) as u32;
        if (chars[i - 1] == 'a' && chars[i] == 'b')
            || (chars[i - 1] == 'c' && chars[i] == 'd')
            || (chars[i - 1] == 'p' && chars[i] == 'q')
            || (chars[i - 1] == 'x' && chars[i] == 'y')
        {
            return false;
        }
    }
    vowels >= 3 && doubles > 0
}

fn nice_v2(s: &&str) -> bool {
    if s.is_empty() {
        return false;
    }
    let chars = s.chars().collect::<Vec<char>>();
    let mut doubles = 0;
    let mut repeating = 0;
    for i in 1..chars.len() {
        for j in (i + 2)..chars.len() {
            if chars[i] == chars[j] && chars[i - 1] == chars[j - 1] {
                doubles += 1;
                break;
            }
        }
        if i >= 2 {
            repeating += (chars[i - 2] == chars[i]) as usize;
        }
    }
    doubles > 0 && repeating > 0
}

fn check_list(list: &str, check: &dyn Fn(&&str) -> bool) -> usize {
    list.lines().filter(check).count()
}

fn main() {
    let input = include_str!("input");
    println!("{}", check_list(input, &nice_v1));
    println!("{}", check_list(input, &nice_v2));
}

#[test]
fn test() {
    assert_eq!(nice_v1(&"ugknbfddgicrmopn"), true);
    assert_eq!(nice_v1(&"ugknbfddgicrmopn"), true);

    assert_eq!(nice_v2(&"qjhvhtzxzqqjkmpb"), true);
    assert_eq!(nice_v2(&"xxyxx"), true);
    assert_eq!(nice_v2(&"uurcxstgmygtbstg"), false);
    assert_eq!(nice_v2(&"ieodomkazucvgmuy"), false);
}
