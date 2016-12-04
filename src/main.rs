use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn room_id(room: String) -> Option<usize> {
    unimplemented!()
}

#[test]
fn simple_rooms() {
    assert_eq!(room_id(String::from("aaaaa-bbb-z-y-x-123[abxyz]")), Some(123));
    assert_eq!(room_id(String::from("(a-b-c-d-e-f-g-h-987[abcde]")), Some(987));
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

