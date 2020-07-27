use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Src {
    Reg(usize),
    Val(i32),
}

fn reg2index(reg: &str) -> Option<usize> {
    match reg {
        "a" | "b" | "c" | "d" => Some((reg.as_bytes()[0] - b'a') as usize),
        _ => None,
    }
}

impl FromStr for Src {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" | "b" | "c" | "d" => Ok(reg2index(s).map(Src::Reg).unwrap()),
            _ => s.parse::<i32>().map(Src::Val),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Inst {
    Cpy { src: Src, offs: Src },
    Jnz { src: Src, offs: Src },
    Out(Src),
    Tgl(usize),
    Inc(usize),
    Dec(usize),
}

impl Inst {
    fn _toggle(&mut self) {
        *self = match *self {
            Inst::Cpy { src, offs } => Inst::Jnz { src, offs },
            Inst::Jnz { src, offs } => Inst::Cpy { src, offs },
            Inst::Out(_) => panic!("Cannot toggle `out` instruction"),
            Inst::Tgl(v) => Inst::Inc(v),
            Inst::Inc(v) => Inst::Dec(v),
            Inst::Dec(v) => Inst::Inc(v),
        }
    }
}

impl FromStr for Inst {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let toks = s.split_whitespace().collect::<Vec<&str>>();
        let inst = match toks[0] {
            "cpy" => Inst::Cpy {
                src: toks[1].parse::<Src>().unwrap(),
                offs: toks[2].parse::<Src>().unwrap(),
            },
            "jnz" => Inst::Jnz {
                src: toks[1].parse::<Src>().unwrap(),
                offs: toks[2].parse::<Src>().unwrap(),
            },
            "out" => Inst::Out(toks[1].parse::<Src>().unwrap()),
            "tgl" => Inst::Tgl(reg2index(toks[1]).unwrap()),
            "inc" => Inst::Inc(reg2index(toks[1]).unwrap()),
            "dec" => Inst::Dec(reg2index(toks[1]).unwrap()),

            _ => panic!("Unknown instruction {}", s),
        };
        Ok(inst)
    }
}

fn run_program(instructions: &[Inst], init: i32) -> Vec<i32> {
    let mut ip = 0i32;
    let mut registers = [init, 0, 0, 0];
    let get_val = |v: Src, regs: &[i32]| -> i32 {
        match v {
            Src::Reg(reg) => regs[reg],
            Src::Val(val) => val,
        }
    };

    let mut output = Vec::new();
    while ip >= 0 && ip < instructions.len() as i32 {
        ip += match instructions[ip as usize] {
            Inst::Cpy { src, offs } => {
                match offs {
                    Src::Reg(r) => {
                        registers[r as usize] = get_val(src, &registers);
                    }
                    Src::Val(_) => panic!("Wat?"),
                }
                1
            }
            Inst::Jnz { src, offs } => {
                if get_val(src, &registers) != 0 {
                    get_val(offs, &registers)
                } else {
                    1
                }
            }
            Inst::Out(src) => {
                output.push(get_val(src, &registers));
                if output.len() >= 100 {
                    return output;
                }
                1
            }
            Inst::Tgl(_r) => unimplemented!(),
            Inst::Inc(r) => {
                registers[r] += 1;
                1
            }
            Inst::Dec(r) => {
                registers[r] -= 1;
                1
            }
        }
    }
    output
}

fn solve(input: &str) -> i32 {
    let instructions = input
        .lines()
        .map(Inst::from_str)
        .map(Result::unwrap)
        .collect::<Vec<Inst>>();

    let reference = (0..100).map(|v| v & 1).collect::<Vec<i32>>();
    for init in 0.. {
        let output = run_program(&instructions, init);
        if output == reference {
            return init;
        }
    }

    panic!("Oh nose!");
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input));
}
