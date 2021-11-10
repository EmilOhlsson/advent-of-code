use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Value {
    Value(u16),
    Variable(String),
}

#[derive(Debug)]
enum Unary {
    Pass(Value),
    Invert(Value),
}

#[derive(Debug)]
enum Node {
    Value(Unary),
    And(Unary, Unary),
    Or(Unary, Unary),
    LShift(Unary, Unary),
    RShift(Unary, Unary),
}

fn parse_var(var: &str) -> Value {
    match var.parse::<u16>() {
        Ok(v) => Value::Value(v),
        Err(_) => Value::Variable(var.to_string()),
    }
}

fn parse_unary(invert: bool, val: Value) -> Unary {
    if invert {
        Unary::Invert(val)
    } else {
        Unary::Pass(val)
    }
}

fn parse(input: &str) -> HashMap<String, Node> {
    let re = Regex::new(
        r"(?m)^(NOT)?\s*(\d+|\w+)\s*(AND|OR|LSHIFT|RSHIFT)?\s*(NOT)?\s*(\d+|\w+)?\s*->\s*(\w+)$",
    )
    .unwrap();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    for cap in re.captures_iter(input) {
        let left = parse_unary(cap.get(1).is_some(), parse_var(&cap[2]));
        if cap.get(3).is_some() {
            let right = parse_unary(cap.get(4).is_some(), parse_var(&cap[5]));
            nodes.insert(
                cap[6].to_string(),
                match &cap[3] {
                    "AND" => Node::And(left, right),
                    "OR" => Node::Or(left, right),
                    "LSHIFT" => Node::LShift(left, right),
                    "RSHIFT" => Node::RShift(left, right),
                    _ => panic!("broken"),
                },
            );
        } else {
            nodes.insert(cap[6].to_string(), Node::Value(left));
        }
    }

    nodes
}

fn resolve_val(
    circuit: &HashMap<String, Node>,
    signals: &mut HashMap<String, u16>,
    val: &Value,
) -> u16 {
    match val {
        Value::Value(val) => *val,
        Value::Variable(var) => {
            let result = resolve(circuit, signals, &var);
            signals.insert(var.to_string(), result);
            result
        }
    }
}

fn resolve_un(
    circuit: &HashMap<String, Node>,
    signals: &mut HashMap<String, u16>,
    val: &Unary,
) -> u16 {
    match val {
        Unary::Pass(v) => resolve_val(circuit, signals, v),
        Unary::Invert(v) => !resolve_val(circuit, signals, v),
    }
}

fn resolve(
    circuit: &HashMap<String, Node>,
    signals: &mut HashMap<String, u16>,
    signal: &str,
) -> u16 {
    if let Some(val) = signals.get(signal) {
        return *val;
    }
    match circuit.get(signal).unwrap() {
        Node::Value(un) => resolve_un(circuit, signals, &un),
        Node::And(l, r) => resolve_un(circuit, signals, &l) & resolve_un(circuit, signals, &r),
        Node::Or(l, r) => resolve_un(circuit, signals, &l) | resolve_un(circuit, signals, &r),
        Node::LShift(l, r) => resolve_un(circuit, signals, &l) << resolve_un(circuit, signals, &r),
        Node::RShift(l, r) => resolve_un(circuit, signals, &l) >> resolve_un(circuit, signals, &r),
    }
}

fn solve_p1(input: &str) -> u16 {
    let circuit = parse(input);
    resolve(&circuit, &mut HashMap::new(), "a")
}

fn solve_p2(input: &str, bypass: u16) -> u16 {
    let circuit = parse(input);
    let mut map = HashMap::new();
    map.insert("b".to_string(), bypass);
    resolve(&circuit, &mut map, "a")
}

fn main() {
    let input = include_str!("input");
    let p1 = solve_p1(input);
    let p2 = solve_p2(input, p1);
    println!("{}, {}", p1, p2);
}
