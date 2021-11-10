fn main() {
    let input = include_str!("input");
    let mut basement = false;
    let floor = input.chars().enumerate().fold(0, |a, (i, c)| {
        if !basement && a < 0 { println!("{}", i); basement = true;}
        match c {
            '(' => a + 1,
            ')' => a - 1,
            _ => panic!("not a parenthesis"),
        }
    });
    println!("{}", floor);
}
