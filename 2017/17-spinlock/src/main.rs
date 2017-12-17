const INSERTIONS: usize = 2017;

fn solve(count: usize) -> (usize, Vec<usize>) {
    let mut rbuffer = vec![0];
    let mut i = 0;
    for v in 1..(INSERTIONS + 1) {
        i = (i + count) % rbuffer.len() + 1;
        rbuffer.insert(i, v);
    }
    (i, rbuffer)
}

fn main() {
    let (i, v) = solve(359);
    println!("{}", v[i + 1]);
}

#[test]
fn test_solution() {
    let (i, v) = solve(3);
    assert_eq!(v[i + 1], 638);
}
