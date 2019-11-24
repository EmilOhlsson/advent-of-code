use md5;

fn solve_p1(secret: &str) -> u32 {
    for i in 1.. {
        let digest = md5::compute(format!("{}{}", secret, i).as_bytes());
        if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xf0 == 0 {
            return i;
        }
    }
    panic!("All borked up");
}

fn solve_p2(secret: &str) -> u32 {
    for i in 1.. {
        let digest = md5::compute(format!("{}{}", secret, i).as_bytes());
        if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {
            return i;
        }
    }
    panic!("All borked up");
}

fn main() {
    println!("{}", solve_p1("iwrupvqb"));
    println!("{}", solve_p2("iwrupvqb"));
}

#[test]
fn test() {
    assert_eq!(solve_p1("abcdef"), 609043);
}
