use std::collections::HashMap;

fn room_checksum(room: &str) -> String {
    let mut chars = HashMap::new();

    for ch in room.chars() {
        if ch != '-' {
            let count = chars.entry(ch).or_insert(0);
            *count += 1;
        }
    }
    let mut list: Vec<(&char, &usize)> = chars.iter().collect();
    list.sort_by(|a, b| {
        if a.1 == b.1 {
            a.0.cmp(b.0)
        } else {
            b.1.cmp(a.1)
        }
    });
    let csum: String = list[0..5].iter().map(|x| *x.0).collect();

    csum
}

fn room_id(room: String) -> Option<isize> {
    let len = room.len();
    let csum_claimed = String::from(&room[len - 6..len - 1]);
    let csum_real = room_checksum(&room[..len - 10]);
    if csum_real == csum_claimed {
        Some(
            String::from(&room[len - 10..len - 7])
                .parse::<isize>()
                .unwrap(),
        )
    } else {
        None
    }
}

#[test]
fn test_checksum() {
    assert_eq!(room_checksum("aaaaa-bbb-z-y-x-"), "abxyz");
    assert_eq!(room_checksum("a-b-c-d-e-f-g-h-"), "abcde");
    assert_eq!(room_checksum("not-a-real-room-"), "oarel");
    assert_ne!(room_checksum("totally-real-room-"), "decoy");
}

#[test]
fn test_id() {
    assert_eq!(
        room_id(String::from("aaaaa-bbb-z-y-x-123[abxyz]")),
        Some(123)
    );
    assert_eq!(
        room_id(String::from("a-b-c-d-e-f-g-h-987[abcde]")),
        Some(987)
    );
    assert_eq!(
        room_id(String::from("not-a-real-room-404[oarel]")),
        Some(404)
    );
    assert_eq!(room_id(String::from("totally-real-room-200[decoy]")), None);
}

#[test]
fn test_decipher() {
    assert_eq!(decipher("qzmt-zixmtkozy-ivhz", 343), "very encrypted name");
}

fn code(c: char, shift: isize) -> char {
    let alen = 'z' as isize - 'a' as isize + 1;
    let mut c_new = c as isize - 'a' as isize + shift;
    c_new = ((c_new % alen) + alen) % alen;
    c_new += 'a' as isize;
    return c_new as u8 as char;
}

#[test]
fn test_code() {
    assert_eq!(code('a', 1), 'b');
    assert_eq!(code('z', 1), 'a');
}

fn decipher(room: &str, shift: isize) -> String {
    room.chars()
        .filter_map(|c| {
            if c.is_alphabetic() {
                Some(code(c, shift))
            } else if c == '-' {
                Some(' ')
            } else {
                None
            }
        }).collect::<String>()
}

fn main() {
    let input = include_str!("../input.txt");

    let mut idsum: isize = 0;
    for line in input.lines() {
        match room_id(line.to_string()) {
            Some(id) => {
                idsum += id;
                println!("{} {}", decipher(&line, id), id);
            }
            _ => (),
        }
    }
    println!("idsum: {}", idsum);
}
