fn power_at(id: i32, x: i32, y: i32) -> i32 {
    assert!(x >= 1 && x <= 300);
    assert!(y >= 1 && y <= 300);
    let rack_id = x + 10;
    let mut level = rack_id * y;
    level += id;
    level *= rack_id;
    level = (level / 100) % 10;
    level -= 5;
    level
}

fn power_sum_at(id: i32, x: i32, y: i32) -> i32 {
    (x..(x+3)).flat_map(|xx| (y..(y+3)).map(move |yy| power_at(id, xx, yy))).sum()
}

fn solve(input: i32) -> (i32, i32) {
    (1..298)
        .flat_map(|x| (1..298).map(move |y| (x, y)))
        .max_by_key(|(x, y)| power_sum_at(input, *x, *y))
        .unwrap()
}

fn main() {
    println!("{:?}", solve(9435));
}

#[test]
fn test() {
    assert_eq!(power_at(8, 3, 5), 4);
    assert_eq!(power_at(57, 122, 79), -5);
    assert_eq!(power_at(39, 217, 196), 0);
    assert_eq!(power_at(71, 101, 153), 4);
    assert_eq!(power_sum_at(18, 33, 45), 29);
    assert_eq!(solve(42), (21, 61));
}
