use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Instr {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

impl Instr {
    fn invertable(&self) -> bool {
        !matches!(*self, Instr::Acc(_))
    }
    fn invert(&mut self) {
        *self = match *self {
            Instr::Acc(arg) => Instr::Acc(arg),
            Instr::Jmp(arg) => Instr::Nop(arg),
            Instr::Nop(arg) => Instr::Jmp(arg),
        };
    }
}

impl FromStr for Instr {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut toks = s.split_whitespace();
        let instr = toks.next().unwrap();
        let arg = toks.next().unwrap().parse::<i64>().unwrap();
        Ok(match instr {
            "acc" => Instr::Acc(arg),
            "jmp" => Instr::Jmp(arg),
            "nop" => Instr::Nop(arg),
            _ => panic!(),
        })
    }
}

fn solve(input: &str) -> i64 {
    let program = input
        .lines()
        .map(Instr::from_str)
        .map(Result::unwrap)
        .collect::<Vec<Instr>>();
    let mut instructions = HashSet::<i64>::new();

    let mut ip = 0i64;
    let mut acc = 0i64;
    while instructions.insert(ip) {
        match program[ip as usize] {
            Instr::Acc(arg) => acc += arg,
            Instr::Jmp(arg) => ip += arg - 1,
            _ => (),
        };
        ip += 1;
    }
    acc
}

fn solve_v2(input: &str) -> i64 {
    let program = input
        .lines()
        .map(Instr::from_str)
        .map(Result::unwrap)
        .collect::<Vec<Instr>>();

    for changed in (0..program.len()).filter(|i| program[*i].invertable()) {
        let mut instructions = HashSet::<i64>::new();
        let mut program_fixed = program.clone();
        program_fixed[changed].invert();

        let mut ip = 0i64;
        let mut acc = 0i64;
        while instructions.insert(ip) {
            if let Some(instruction) = program_fixed.get(ip as usize) {
                match instruction {
                    Instr::Acc(arg) => acc += arg,
                    Instr::Jmp(arg) => ip += arg - 1,
                    _ => (),
                };
                ip += 1;
            } else {
                // Program exited normally
                return acc;
            }
        }
    }

    panic!("Did not find solution");
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input));
    println!("{}", solve_v2(input));
}
