fn count_pairs(mut a: usize, mut b: usize) -> usize {
    let mut count = 0;
    for _ in 0..40_000_000 {
        a = (a *16807) % 2147483647;
        b = (b * 48271) % 2147483647;

        if (a & 0xffff) == (b & 0xffff) { count += 1;}
    }

    return count;
}

fn main() {
    println!("{}", count_pairs(618, 814));
}

#[test]
fn test_generators() {
    assert_eq!(count_pairs(65, 8921), 588);
}
