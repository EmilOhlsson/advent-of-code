use std::collections::HashMap;

fn get_value(regs: &mut HashMap<String, isize>, r: &String) -> isize {
    match r.parse::<isize>() {
        Ok(v) => v,
        Err(_) => {
            let v = regs.entry(r.clone()).or_insert(0);
            *v
        }
    }
}

fn run_program(input: &str) -> isize {
    let mut regs = HashMap::<String, isize>::new();
    let mut sound = 0;
    let instructions = input.lines().collect::<Vec<&str>>();
    let mut pc = 0;


    loop {
        let mut inc = 1;
        if let Some(line) = instructions.get(pc as usize) {
            let toks = line.split_whitespace().map(String::from).collect::<Vec<String>>();
            match toks[0].as_str() {
                "snd" => {
                    let v = get_value(&mut regs, &toks[1]);
                    sound = v;
                }
                "set" => {
                    let v = get_value(&mut regs, &toks[2]);
                    regs.insert(toks[1].clone(), v);
                }
                "add" => {
                    let y = get_value(&mut regs, &toks[2]);
                    let x = regs.entry(toks[1].clone()).or_insert(0);
                    *x += y;
                }
                "mul" => {
                    let y = get_value(&mut regs, &toks[2]);
                    let x = regs.entry(toks[1].clone()).or_insert(0);
                    *x *= y;
                }
                "mod" => {
                    let y = get_value(&mut regs, &toks[2]);
                    let x = regs.entry(toks[1].clone()).or_insert(0);
                    *x %= y;
                }
                "rcv" => {
                    let x = get_value(&mut regs, &toks[1]);
                    if x != 0 {
                        if sound != 0 {
                            return sound;
                        }
                    }
                }
                "jgz" => {
                    let x = get_value(&mut regs, &toks[1]);
                    let y = get_value(&mut regs, &toks[2]);
                    if x > 0 {
                        inc = y;
                    }
                }
                _ => panic!("Erhmagawd... {}", toks[0]),
            }
            pc += inc;
        }
    }
}

fn main() {
    let program = include_str!("input.txt");
    println!("{}", run_program(program));
}

#[test]
fn test_program() {
    let program = include_str!("input-simple.txt");
    assert_eq!(run_program(program), 4);
}
