use std::fs::File;
use std::io::{BufRead,BufReader};
use std::io::Result;

enum Src {
    Reg(usize),
    Val(isize),
}

enum Inst {
    Cpy {
        src: Src,
        dst: usize,
    },
    Jnz {
        src: usize,
        offs: isize,
    },
    Inc(usize),
    Dec(usize),
}

fn parse_line(line: std::io::Result<String>) -> Inst {
    let lineres = line.unwrap();
    let toks: Vec<&str> = lineres.split_whitespace().collect();
    match toks[0] {
        "cpy" => Inst::Cpy {
            src: match toks[1] {
                "a" | "b" | "c" | "d" => Src::Reg((toks[1].as_bytes()[0] - 'a' as u8) as usize),
                _ => Src::Val(toks[1].parse::<isize>().unwrap())
            },
            dst: (toks[1].as_bytes()[0] - 'a' as u8) as usize,
        },
        "jnz" => Inst::Jnz {
            src: (toks[1].as_bytes()[0] - 'a' as u8) as usize,
            offs: toks[2].parse::<isize>().unwrap(),
        },
        "inc" => Inst::Inc((toks[1].as_bytes()[0] - 'a' as u8) as usize),
        "dec" => Inst::Dec((toks[1].as_bytes()[0] - 'a' as u8) as usize),
        _ => panic!("Token not recognized: {}", toks[0]),
    }
}

fn main() {
    let input = match File::open("input.txt") {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("Unable to open input.txt: {}", e),
    };
    let program: Vec<Inst> = input.lines().map(parse_line).collect();
}
