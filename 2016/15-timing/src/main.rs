
/// Take a list of (positions, start)
fn find_time(ss: &Vec<(usize, usize)>) -> usize {
    for i in 0.. {
        let mut found = true;
        for (j, (p, s)) in ss.iter().enumerate() {
            let pos = (s + i + j + 1) %  p;
            if pos != 0 {
                found = false;
            }
        }
        if found { return i; }
    }
    panic!();
}

fn main() {
    let states = vec![(13, 11), (5, 0), (17, 11), (3, 0), (7, 2), (19, 17)];
    println!("{}", find_time(&states));
}

#[test]
fn test_simple() {
    let states = vec![(5, 4), (2, 1)];
    assert_eq!(find_time(&states), 5);
}
