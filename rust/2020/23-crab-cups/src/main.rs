fn move_cups(current: usize, cups: &mut [usize]) -> usize {
    let len = cups.len() - 1;

    // Take three after the current
    let a = cups[current];
    let b = cups[a];
    let c = cups[b];

    // And remove them
    cups[current] = cups[c];

    // Find out where to place them
    let mut dst = (current + len - 2) % len + 1;
    while dst == a || dst == b || dst == c {
        // Yuck
        dst = (dst + cups.len() - 1 - 2) % (cups.len() - 1) + 1;
    }

    // And place them
    let tmp = cups[dst];
    cups[dst] = a;
    cups[c] = tmp;

    // Next cup is the one after the current
    cups[current]
}

fn as_string(start: usize, length: usize, cups: &[usize]) -> String {
    let mut sequence = Vec::new();
    let mut current = start;
    for _ in 0..length {
        let v = cups[current];
        sequence.push(v);
        current = v;
    }
    sequence
        .iter()
        .map(|v| format!("{}", v))
        .collect::<String>()
}

fn play(cups_next: &mut [usize], start: usize, turns: u32) {
    let mut current = start;
    for _move in 1..=turns {
        current = move_cups(current, cups_next);
    }
}

fn solve_v1(input: &str, turns: u32) -> String {
    let cups = input
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    let mut cups_next = vec![0usize; cups.len() + 1];
    for i in 0..cups.len() {
        cups_next[cups[i]] = cups[(i + 1) % cups.len()];
    }
    play(&mut cups_next, cups[0], turns);
    as_string(1, cups.len() - 1, &cups_next)
}

fn solve_v2(input: &str, turns: u32) -> u64 {
    let mut cups = input
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    // Add the rest of the cups
    for i in cups.len() + 1..=1_000_000 {
        cups.push(i);
    }

    // And build mapping from cup to neighbor
    let mut cups_next = vec![0usize; cups.len() + 1];
    for i in 0..cups.len() {
        cups_next[cups[i]] = cups[(i + 1) % cups.len()];
    }
    play(&mut cups_next, cups[0], turns);
    let a = cups_next[1] as u64;
    let b = cups_next[cups_next[1]] as u64;
    a * b
}

fn main() {
    println!("{}", solve_v1("469217538", 100));
    println!("{}", solve_v2("469217538", 10_000_000));
}

#[test]
fn test() {
    assert_eq!(solve_v1("389125467", 10), "92658374");
    assert_eq!(solve_v1("389125467", 100), "67384529");
}
