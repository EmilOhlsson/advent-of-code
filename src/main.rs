use std::fs::File;
use std::io::{BufRead, BufReader};
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
    list.sort_by(|a, b| if a.1 == b.1 { a.0.cmp(b.0) } else { b.1.cmp(a.1) });
    let csum: String = list[0..5].iter().map(|x| *x.0).collect();

    csum
}

fn room_id(room: String) -> Option<usize> {
    let len = room.len();
    let csum_claimed = String::from(&room[len-6..len-1]);
    let csum_real = room_checksum(&room[..len-10]);
    if csum_real == csum_claimed {
        Some(String::from(&room[len-10..len-7]).parse::<usize>().unwrap())
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
    assert_eq!(room_id(String::from("aaaaa-bbb-z-y-x-123[abxyz]")), Some(123));
    assert_eq!(room_id(String::from("a-b-c-d-e-f-g-h-987[abcde]")), Some(987));
    assert_eq!(room_id(String::from("not-a-real-room-404[oarel]")), Some(404));
    assert_eq!(room_id(String::from("totally-real-room-200[decoy]")), None);
}

fn main() {
    let reader = match File::open("input.txt") {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("Unable to open file input.txt"),
    };

    let mut idsum: usize = 0; 
    for line in reader.lines() {
        match room_id(line.unwrap()) {
            Some(id) => idsum += id,
            _ => (),
        }
    }
    println!("idsum: {}", idsum);
}

