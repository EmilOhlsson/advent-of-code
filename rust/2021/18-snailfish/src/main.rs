use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
enum SnailNumber {
    Value(u64),
    Pair(Box<SnailNumber>, Box<SnailNumber>),
}

impl fmt::Display for SnailNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SnailNumber::Value(v) => write!(f, "{}", v),
            SnailNumber::Pair(a, b) => write!(f, "[{},{}]", a, b),
        }
    }
}

impl FromStr for SnailNumber {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack: Vec<Vec<SnailNumber>> = vec![vec![]];
        let mut number = 0u64;
        let mut in_number = false;
        for ch in s.chars() {
            match ch {
                '[' => stack.push(Vec::new()),
                ']' => {
                    if in_number {
                        stack.last_mut().unwrap().push(SnailNumber::Value(number));
                        in_number = false;
                        number = 0;
                    }
                    let completed = stack.pop().unwrap();
                    let top = stack.last_mut().unwrap();
                    top.push(SnailNumber::Pair(
                        Box::new(completed[0].clone()),
                        Box::new(completed[1].clone()),
                    ));
                }
                ',' => {
                    let top = stack.last_mut().unwrap();
                    if in_number {
                        top.push(SnailNumber::Value(number));
                        in_number = false;
                        number = 0;
                    }
                }
                _ => {
                    let n = ch.to_digit(10).unwrap() as u64;
                    number *= 10;
                    number += n;
                    in_number = true;
                }
            }
        }
        Ok(stack[0].last().unwrap().clone())
    }
}

impl SnailNumber {
    fn add(self, other: SnailNumber) -> SnailNumber {
        SnailNumber::Pair(Box::new(self), Box::new(other))
    }

    fn as_number(&self) -> u64 {
        if let SnailNumber::Value(v) = self {
            *v
        } else {
            panic!("{} is not a number", self);
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            SnailNumber::Value(v) => *v,
            SnailNumber::Pair(a, b) => 3 * a.magnitude() + 2 * b.magnitude(),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            SnailNumber::Value(v) => {
                if *v >= 10 {
                    let left = SnailNumber::Value(*v / 2);
                    let right = SnailNumber::Value((*v + 1) / 2);
                    *self = SnailNumber::Pair(Box::new(left), Box::new(right));
                    true
                } else {
                    false
                }
            }
            SnailNumber::Pair(l, r) => {
                if l.split() {
                    return true;
                }
                if r.split() {
                    return true;
                }
                false
            }
        }
    }

    fn inc_left(&mut self, inc: u64) {
        match self {
            SnailNumber::Value(v) => *v += inc,
            SnailNumber::Pair(l, _) => l.inc_left(inc),
        }
    }

    fn inc_right(&mut self, inc: u64) {
        match self {
            SnailNumber::Value(v) => *v += inc,
            SnailNumber::Pair(_, r) => r.inc_right(inc),
        }
    }

    fn explode(&mut self, level: u32) -> (bool, Option<u64>, Option<u64>) {
        match self {
            SnailNumber::Value(_) => (false, None, None),
            SnailNumber::Pair(left, right) => {
                if level == 4 {
                    let lv = left.as_number();
                    let rv = right.as_number();
                    *self = SnailNumber::Value(0);
                    (true, Some(lv), Some(rv))
                } else {
                    let (exploded, a, tmp) = left.explode(level + 1);
                    if let Some(v) = tmp {
                        right.inc_left(v);
                    }
                    if exploded {
                        return (true, a, None);
                    }

                    let (exploded, tmp, b) = right.explode(level + 1);
                    if let Some(v) = tmp {
                        left.inc_right(v);
                    }
                    (exploded, None, b)
                }
            }
        }
    }

    fn get_depth(&self) -> u32 {
        match self {
            SnailNumber::Value(_) => 0,
            SnailNumber::Pair(l, r) => 1 + std::cmp::max(l.get_depth(), r.get_depth()),
        }
    }

    fn get_max(&self) -> u64 {
        match self {
            SnailNumber::Value(v) => *v,
            SnailNumber::Pair(l, r) => std::cmp::max(l.get_max(), r.get_max()),
        }
    }

    fn reduce(&mut self) -> &Self {
        loop {
            let max = self.get_max();
            let depth = self.get_depth();
            if depth > 4 {
                self.explode(0);
            } else if max >= 10 {
                self.split();
            } else {
                break;
            }
        }
        self
    }
}

fn parse_line(input: &str) -> SnailNumber {
    SnailNumber::from_str(input).unwrap()
}

fn parse(input: &str) -> Vec<SnailNumber> {
    input.lines().map(parse_line).collect()
}

fn solve_p1(input: &str) -> u64 {
    let numbers = parse(input);
    numbers
        .iter()
        .cloned()
        .reduce(|acc, item| {
            let mut num = acc.add(item);
            num.reduce();
            num
        })
        .unwrap()
        .magnitude()
}

fn solve_p2(input: &str) -> u64 {
    let numbers = parse(input);
    let mut max = 0;
    for (x, n1) in numbers.iter().enumerate() {
        for (y, n2) in numbers.iter().enumerate() {
            if x != y {
                let mut result = n1.clone().add(n2.clone());
                result.reduce();
                max = std::cmp::max(max, result.magnitude());
            }
        }
    }
    max
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve_p1(input));
    println!("{}", solve_p2(input));
}

#[test]
fn test_trivial() {
    let left = parse_line("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let right = parse_line("[1,1]");
    let mut sum = left.add(right);
    sum.reduce();
    assert_eq!(format!("{}", sum), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
}

#[test]
fn test_larger() {
    let input = include_str!("input-simple");
    let nums = parse(input);
    let mut num = nums[0].clone();
    for n in nums.iter().skip(1) {
        println!("  {}", num);
        println!("+ {}", n);
        num = num.add(n.clone());
        num.reduce();
        println!("= {}", num);
        println!();
    }
    assert_eq!(
        format!("{}", num),
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
    );
}

#[test]
fn test_homework() {
    let input = include_str!("input-homework");
    let nums = parse(input);
    let mut num = nums[0].clone();
    for n in nums.iter().skip(1) {
        num = num.add(n.clone());
        num.reduce();
    }
    assert_eq!(
        format!("{}", num),
        "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
    );
    assert_eq!(num.magnitude(), 4140);
    assert_eq!(solve_p1(input), 4140);
}

#[test]
fn test_parsing() {
    use SnailNumber::{Pair, Value};
    let number = parse_line("[[1,2],3]");
    assert_eq!(
        number,
        Pair(
            Box::new(Pair(Box::new(Value(1)), Box::new(Value(2)))),
            Box::new(Value(3))
        )
    );
}

#[test]
fn test_explode() {
    assert_eq!(
        format!("{}", {
            let mut num = parse_line("[[[[[9,8],1],2],3],4]");
            num.explode(0);
            num
        }),
        "[[[[0,9],2],3],4]"
    );
    assert_eq!(
        format!("{}", {
            let mut num = parse_line("[7,[6,[5,[4,[3,2]]]]]");
            num.explode(0);
            num
        }),
        "[7,[6,[5,[7,0]]]]"
    );
    assert_eq!(
        format!("{}", {
            let mut num = parse_line("[[6,[5,[4,[3,2]]]],1]");
            num.explode(0);
            num
        }),
        "[[6,[5,[7,0]]],3]"
    );
    assert_eq!(
        format!("{}", {
            let mut num = parse_line("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
            num.explode(0);
            num
        }),
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
    );
}

#[test]
fn test_split() {
    assert_eq!(
        format!("{}", {
            let mut num = parse_line("[10,11]");
            num.split();
            num
        }),
        "[[5,5],11]"
    );
    assert_eq!(
        format!("{}", {
            let mut num = parse_line("[[5,5],11]");
            num.split();
            num
        }),
        "[[5,5],[5,6]]"
    );
}

#[test]
fn test_magnitude() {
    assert_eq!(parse_line("[9,1]").magnitude(), 29);
    assert_eq!(parse_line("[1,9]").magnitude(), 21);
    assert_eq!(parse_line("[[9,1],[1,9]]").magnitude(), 129);
}
