fn solve(input: &str, rows: usize) -> usize {
    let mut traps = 0u128;
    let mask = (1u128 << input.len()) - 1;

    // Parse
    for c in input.chars() {
        traps = (traps << 1) | (c == '^') as u128;
    }

    let mut safe = 0;
    for _ in 0..rows {
        safe += (mask ^ traps).count_ones() as usize;
        traps = ((traps << 1) ^ (traps >> 1)) & mask;
    }

    safe
}

fn main() {
    let input = include_str!("input.txt").trim();
    println!("{}", solve(input, 40));
    println!("{}", solve(input, 400_000));
}

#[test]
fn test_p1() {
    assert_eq!(solve("..^^.", 3), 6);
    assert_eq!(solve(".^^.^.^^^^", 10), 38);
}
