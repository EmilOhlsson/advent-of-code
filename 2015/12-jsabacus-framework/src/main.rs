use serde_json::Value;

fn recursive_sum(value: &Value) -> i64 {
    match value {
        Value::Number(num) => num.as_i64().unwrap(),
        Value::Array(vec) => vec.iter().fold(0, |acc, e| acc + recursive_sum(e)),
        Value::Object(map) => map.values().fold(0, |acc, e| acc + recursive_sum(e)),
        _ => 0,
    }
}

fn is_red(val: &Value) -> bool {
    if let Value::String(s) = val {
        s == "red"
    } else {
        false
    }
}

fn recursive_sum_v2(value: &Value) -> i64 {
    match value {
        Value::Number(num) => num.as_i64().unwrap(),
        Value::Array(vec) => vec.iter().fold(0, |acc, e| acc + recursive_sum_v2(e)),
        Value::Object(map) => {
            if !map.values().any(is_red) {
                map.values().fold(0, |acc, e| acc + recursive_sum_v2(e))
            } else {
                0
            }
        }
        _ => 0,
    }
}

fn solve(input: &str) -> i64 {
    recursive_sum(&serde_json::from_str(input).unwrap())
}

fn solve_v2(input: &str) -> i64 {
    recursive_sum_v2(&serde_json::from_str(input).unwrap())
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input));
    println!("{}", solve_v2(input));
}

#[test]
fn test() {
    assert_eq!(solve("[1,2,3]"), 6);
    assert_eq!(solve("{\"a\":2,\"b\":4}"), 6);
    assert_eq!(solve("{\"a\":{\"b\":4},\"c\":-1}"), 3);

    assert_eq!(solve_v2("[1,{\"c\":\"red\",\"b\":2},3]"), 4);
}
