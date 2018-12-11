fn power_at(id: i64, x: i64, y: i64) -> i64 {
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

fn power_sum_at(id: i64, s: i64, x: i64, y: i64) -> i64 {
    (x..(x+s)).flat_map(|xx| (y..(y+s)).map(move |yy| power_at(id, xx, yy))).sum()
}

fn solve(input: i64, size: i64) -> ((i64, i64), i64) {
    (1..=(300 - size))
        .flat_map(|x| (1..=(300 - size)).map(move |y| ((x, y), power_sum_at(input, size, x, y))))
        .max_by_key(|(_, p)| *p)
        .unwrap_or(((0,0), 0))
}

fn solve_p2(input: i64) -> (((i64, i64), i64), i64) {
    (1..=300).map(|s| (solve(input, s), s)).max_by_key(|((_, p), _)| *p).unwrap()
}

fn main() {
    println!("{:?}", solve(9435, 3));
    println!("{:?}", solve_p2(9435));
}

#[test]
fn test() {
    assert_eq!(power_at(8, 3, 5), 4);
    assert_eq!(power_at(57, 122, 79), -5);
    assert_eq!(power_at(39, 217, 196), 0);
    assert_eq!(power_at(71, 101, 153), 4);
    assert_eq!(power_sum_at(18, 3, 33, 45), 29);
    assert_eq!(solve(42, 3), (21, 61));
}
