pub mod intmachine;
use intmachine::Intmachine;

fn solve(input: &str, part: i64) -> i64 {
    let mut machine = Intmachine::load(input);
    let output = machine.run(&[part]);
    output[0]
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input, 1));
    println!("{}", solve(input, 2));
}

#[test]
fn rb_test() {
    let mut machine = Intmachine::load("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    let output = machine.run(&[]);
    assert_eq!(
        output,
        vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
    );
}

#[test]
fn bignum_test() {
    let mut machine = Intmachine::load("1102,34915192,34915192,7,4,7,99,0");
    let output = machine.run(&[]);
    assert_eq!(output, vec![1219070632396864])
}

#[test]
fn test0() {
    let mut machine = Intmachine::load("104,1125899906842624,99");
    let output = machine.run(&[]);
    assert_eq!(output[0], 1125899906842624);
}
