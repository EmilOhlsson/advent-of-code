const INSERTIONS: usize = 50_000_000;

fn solve(count: usize) -> usize {
    let mut i = 0;
    let mut first = None;
    for v in 1..(INSERTIONS + 1) {
        i = (i + count) % v + 1;
        if i == 1 { first = Some(v); }
    }
    return first.unwrap();
}

fn main() {
    let v = solve(359);
    println!("{}", v);
}

#[test]
fn test_solution() {
    let (i, v) = solve(3);
    assert_eq!(v[i + 1], 638);
}
