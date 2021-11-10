fn seat_id(seat: &str) -> u32 {
    let mut row = (0, 127);
    let mut col = (0, 7);
    for ch in seat.chars() {
        match ch {
            'F' => row = (row.0, (row.0 + row.1) / 2),
            'B' => row = ((row.0 + row.1) / 2 + 1, row.1),
            'L' => col = (col.0, (col.0 + col.1) / 2),
            'R' => col = ((col.0 + col.1) / 2 + 1, col.1),
            _ => panic!()
        }
    }

    row.0 * 8 + col.0
}

fn solve(input: &str) -> (u32, u32) {
    let mut ids = input.lines().map(seat_id).collect::<Vec<_>>();
    ids.sort_unstable();
    
    for pair in ids.windows(2) {
        if pair[1] != pair[0] + 1 {
            return (*ids.last().unwrap(), pair[0] + 1);
        }
    }
    panic!("Did not find empty seat");
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input));
}

#[test]
fn test() {
    assert_eq!(seat_id("FBFBBFFRLR"), 357);
    assert_eq!(seat_id("BFFFBBFRRR"), 567);
    assert_eq!(seat_id("FFFBBBFRRR"), 119);
    assert_eq!(seat_id("BBFFBBFRLL"), 820);
}
