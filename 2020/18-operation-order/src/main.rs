#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}

fn evaluate(expression: &str) -> u64 {
    let mut num_stack = vec![0];
    let mut op_stack: Vec<Option<Op>> = vec![None];
    for ch in expression.chars() {
        match ch {
            '0'..='9' => {
                let val = ch.to_digit(10).unwrap() as u64;
                let top = num_stack.last_mut().unwrap();
                if let Some(op) = op_stack.last().unwrap() {
                    match op {
                        Op::Add => *top += val,
                        Op::Mul => *top *= val,
                    }
                } else {
                    *top = val;
                }
            }
            '+' => *op_stack.last_mut().unwrap() = Some(Op::Add),
            '*' => *op_stack.last_mut().unwrap() = Some(Op::Mul),
            '(' => {
                num_stack.push(0);
                op_stack.push(None);
            }
            ')' => {
                let val = num_stack.pop().unwrap();
                let top = num_stack.last_mut().unwrap();
                op_stack.pop();
                if let Some(op) = op_stack.last().unwrap() {
                    match op {
                        Op::Add => *top += val,
                        Op::Mul => *top *= val,
                    }
                } else {
                    *top = val;
                }
            }
            ' ' => (),
            _ => panic!("Did not recognize {}", ch),
        }
    }
    num_stack[0]
}

fn solve_p1(input: &str) -> u64 {
    input.lines().map(evaluate).sum()
}

type Expr = Vec<Vec<u64>>;

fn calculate(expr: &Expr) -> u64 {
    expr.iter().map(|e| e.iter().sum::<u64>()).product()
}

fn push_val(operation: Option<Op>, val: u64, expr: &mut Expr) {
    if let Some(op) = operation {
        match op {
            Op::Add => expr.last_mut().unwrap().push(val),
            Op::Mul => expr.push(vec![val]),
        }
    } else {
        expr.push(vec![val]);
    }
}

fn evaluate_p2(expression: &str) -> u64 {
    let mut num_stack = vec![Expr::new()];
    let mut expr_stack: Vec<Option<Op>> = vec![None];
    for ch in expression.chars() {
        match ch {
            '0'..='9' => {
                let val = ch.to_digit(10).unwrap() as u64;
                let expr = num_stack.last_mut().unwrap();
                push_val(*expr_stack.last().unwrap(), val, expr);
            }
            '+' => *expr_stack.last_mut().unwrap() = Some(Op::Add),
            '*' => *expr_stack.last_mut().unwrap() = Some(Op::Mul),
            '(' => {
                num_stack.push(Vec::new());
                expr_stack.push(None);
            }
            ')' => {
                let val = calculate(&num_stack.pop().unwrap());
                let expr = num_stack.last_mut().unwrap();
                expr_stack.pop();
                push_val(*expr_stack.last().unwrap(), val, expr);
            }
            ' ' => (),
            _ => panic!("Did not recognize {}", ch),
        }
    }
    calculate(&num_stack[0])
}

fn solve_p2(input: &str) -> u64 {
    input.lines().map(evaluate_p2).sum()
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve_p1(input));
    println!("{}", solve_p2(input));
}

#[test]
fn test() {
    assert_eq!(evaluate("1 + 2 * 3 + 4 * 5 + 6"), 71);
    assert_eq!(evaluate("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    assert_eq!(evaluate("2 * 3 + (4 * 5)"), 26);
    assert_eq!(evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
    assert_eq!(evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
    assert_eq!(evaluate("(1)"), 1);
}
