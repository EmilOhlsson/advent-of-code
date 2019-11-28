fn eval_v1(line: &str) -> u32 {
    let mut size = 0;
    let mut escape = false;
    let mut skip = 0;

    for c in line.chars() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        if escape {
            skip = match c {
                'x' => 2,
                _ => 0,
            };
            escape = false;
            size += 1;
        } else {
            escape = match c {
                '\\' => true,
                _ => false,
            };
            if !escape {
                size += 1;
            }
        }
    }
    line.len() as u32 + 2 - size
}

fn eval_v2(line: &str) -> u32 {
    let mut new_string = String::new();
    new_string.push('"');
    for c in line.chars() {
        match c {
            '"' => new_string.push_str("\\\""),
            '\\' => new_string.push_str("\\\\"),
            _ => new_string.push(c),
        }
    }
    new_string.push('"');
    eval_v1(&new_string)
}

fn solve(input: &str) -> (u32, u32) {
    (
        input.lines().map(eval_v1).sum(),
        input.lines().map(eval_v2).sum(),
    )
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input));
}

#[test]
fn test() {
    assert_eq!(eval_v1("\"\""), 2);
    assert_eq!(eval_v1("\"abc\""), 2);
    assert_eq!(eval_v1("\"aaa\\\"aaa\""), 3);
    assert_eq!(eval_v1("\"\\x27\""), 5);

    assert_eq!(eval_v2("\"\""), 6 - 2);
    assert_eq!(eval_v2("\"abc\""), 9 - 5);
    assert_eq!(eval_v2("\"aaa\\\"aaa\""), 16 - 10);
    assert_eq!(eval_v2("\"\\x27\""), 11 - 6);
}
