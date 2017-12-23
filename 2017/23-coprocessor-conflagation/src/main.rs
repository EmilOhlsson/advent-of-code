use std::collections::HashMap;

fn get<'a>(regs: &mut HashMap<&'a str, isize>, r: &'a str) -> isize {
    match r.parse::<isize>() {
        Ok(v) => v,
        Err(_) => *regs.entry(r).or_insert(0),
    }
}

fn emulate(input: &str) -> usize {
    let mut muls = 0;
    let mut registers: HashMap<&str, isize> = HashMap::new();
    let instructions = input.lines().collect::<Vec<&str>>();
    let mut i = 0;

    while let Some(line) = instructions.get(i as usize) {
        let ts = line.split_whitespace().collect::<Vec<&str>>();
        match ts[0] {
            "set" => {
                let y = get(&mut registers, ts[2]);
                let x = registers.entry(ts[1]).or_insert(0);
                *x = y;
            }
            "sub" => {
                let y = get(&mut registers, ts[2]);
                let x = registers.entry(ts[1]).or_insert(0);
                *x -= y;
            }
            "mul" => {
                muls += 1;
                let y = get(&mut registers, ts[2]);
                let x = registers.entry(ts[1]).or_insert(0);
                *x *= y;
            }
            "jnz" => {
                let y = get(&mut registers, ts[2]);
                let x = get(&mut registers, ts[1]);
                if x != 0 {
                    i += y;
                    continue;
                }
            }
            _ => panic!("AAAAaaargh: {}", ts[0]),
        }
        i += 1;
    }

    muls
}

fn main() {
    let input = include_str!("input");
    println!("{}", emulate(input));
}
