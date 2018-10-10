fn fold(bss: &str) -> String {
    let rev = bss
        .chars()
        .rev()
        .map(|c| if c == '0' { '1' } else { '0' })
        .collect::<String>();
    format!("{}0{}", bss, rev)
}

fn checksum(bss: &str) -> String {
    let mut result = bss.to_owned();
    while result.len() % 2 == 0 {
        let tmp = result.chars().collect::<Vec<char>>();
        result = tmp
            .chunks(2)
            .map(|ch| if ch[0] == ch[1] { '1' } else { '0' })
            .collect::<String>()
    }

    result
}

fn solve(bss: &str, len: usize) -> String {
    let mut data = bss.to_owned();
    while data.len() < len {
        data = fold(&data);
    }

    data.truncate(len);

    checksum(&data)
}

fn main() {
    println!("{}", solve("11100010111110100", 272));
}

#[test]
fn test_simple() {
    assert_eq!(fold("1"), "100");
    assert_eq!(fold("0"), "001");
    assert_eq!(fold("11111"), "11111000000");
    assert_eq!(fold("111100001010"), "1111000010100101011110000");

    assert_eq!(checksum("10"), "0");
    assert_eq!(checksum("11"), "1");
    assert_eq!(checksum("110010110100"), "100");

    assert_eq!(solve("10000", 20), "01100");
}
