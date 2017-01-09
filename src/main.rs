use std::fs::File;
use std::io::{BufRead, BufReader};

enum Src {
    Reg(usize),
    Val(isize),
}

enum Inst {
    Cpy { src: Src, dst: usize },
    Jnz { src: Src, offs: isize },
    Inc(usize),
    Dec(usize),
}

fn reg2index(reg: &str) -> usize {
    (reg.as_bytes()[0] - 'a' as u8) as usize
}

fn str2src(src: &str) -> Src {
    match src {
        "a" | "b" | "c" | "d" => Src::Reg(reg2index(src)),
        _ => Src::Val(src.parse::<isize>().unwrap()),
    }
}

fn parse_line(line: std::io::Result<String>) -> Inst {
    let lineres = line.unwrap();
    let toks: Vec<&str> = lineres.split_whitespace().collect();
    match toks[0] {
        "cpy" => {
            Inst::Cpy {
                src: str2src(toks[1]),
                dst: reg2index(toks[2]),
            }
        }
        "jnz" => {
            Inst::Jnz {
                src: str2src(toks[1]),
                offs: toks[2].parse::<isize>().unwrap(),
            }
        }
        "inc" => Inst::Inc(reg2index(toks[1])),
        "dec" => Inst::Dec(reg2index(toks[1])),
        _ => panic!("Token not recognized: {}", toks[0]),
    }
}

fn main() {
    let input = match File::open("input.txt") {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("Unable to open input.txt: {}", e),
    };
    let program: Vec<Inst> = input.lines().map(parse_line).collect();

    let mut pc = 0usize;
    let mut registers = [0, 0, 1, 0];
    while pc < program.len() {
        match program[pc] {
            Inst::Cpy { ref src, dst } => {
                registers[dst] = match *src {
                    Src::Reg(reg) => registers[reg],
                    Src::Val(val) => val,
                }
            }
            Inst::Jnz { ref src, offs } => {
                if match *src {
                    Src::Reg(i) => registers[i],
                    Src::Val(val) => val,
                } != 0 {
                    pc = (pc as isize + offs) as usize;
                    continue;
                }
            }
            Inst::Inc(reg) => registers[reg] += 1,
            Inst::Dec(reg) => registers[reg] -= 1,
        };
        pc += 1;
    }
    println!("register a: {}", registers[0]);
}
