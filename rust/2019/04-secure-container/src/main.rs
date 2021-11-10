fn is_valid(val: &i32) -> bool {
    let mut tmp = *val;
    let mut digits = Vec::new();
    while tmp > 0 {
        let digit = tmp % 10;
        if let Some(&prev) = digits.last() {
            if digit > prev {
                return false;
            }
        }
        digits.push(digit);
        tmp /= 10;
    }

    let mut double = false;
    for i in 1..digits.len() {
        double |= digits[i] == digits[i - 1];
    }
    double
}

fn is_valid_v2(val: &i32) -> bool {
    let mut tmp = *val;
    let mut digits = Vec::new();
    while tmp > 0 {
        let digit = tmp % 10;
        if let Some(&prev) = digits.last() {
            if digit > prev {
                return false;
            }
        }
        digits.push(digit);
        tmp /= 10;
    }

    let mut double = false;
    for i in 1..digits.len() {
        let couple = digits[i] == digits[i - 1];
        if couple {
            let next = digits.get(i + 1).unwrap_or(&-1);
            let prev = digits.get((i as i32 - 2) as usize).unwrap_or(&-1);
            if &digits[i] != next && &digits[i] != prev {
                double = true;
            }
        }
    }
    double
}

fn solve(low: i32, high: i32, valid: &dyn Fn(&i32) -> bool) -> usize {
    (low..=high).filter(valid).count()
}

fn main() {
    let low = 168630;
    let high = 718098;
    println!("{}", solve(low, high, &is_valid));
    println!("{}", solve(low, high, &is_valid_v2));
}

#[test]
fn test() {
    assert_eq!(is_valid(&111111), true);
    assert_eq!(is_valid(&223450), false);
    assert_eq!(is_valid(&123789), false);

    assert_eq!(is_valid_v2(&112233), true);
    assert_eq!(is_valid_v2(&123444), false);
    assert_eq!(is_valid_v2(&111122), true);
    assert_eq!(is_valid_v2(&168889), false);
}
