fn look_and_say(input: &str) -> String {
    let mut result = String::new();
    let mut count = 1;
    let mut prev = input.chars().next().unwrap();
    for ch in input.chars().skip(1) {
        if ch == prev {
            count += 1;
        } else {
            result += &format!("{}{}", count, prev);
            prev = ch;
            count = 1;
        }
    }
    format!("{}{}{}", result, count, prev)
}

fn solve(input: &str, count: u32) -> usize {
    let mut result = look_and_say(input);
    for _ in 1..count {
        result = look_and_say(&result);
    }
    result.len()
}

fn main() {
    let input = "1113122113";
    println!("{}", solve(input, 40));
    println!("{}", solve(input, 50));
}

#[test]
fn test() {
    assert_eq!(look_and_say("1"), "11");
    assert_eq!(look_and_say("2"), "12");
    assert_eq!(look_and_say("11"), "21");
    assert_eq!(look_and_say("21"), "1211");
    assert_eq!(look_and_say("1211"), "111221");
    assert_eq!(look_and_say("111221"), "312211");
}
