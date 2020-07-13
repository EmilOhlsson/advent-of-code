fn solve(input: &str, initial: i32) -> i32 {
    let mut a = initial;
    let mut b = 0i32;
    let instructions = input.lines().map(String::from).collect::<Vec<String>>();
    let mut ip = 0i32;
    while let Some(instruction) = instructions.get(ip as usize) {
        let toks = instruction.split_whitespace().collect::<Vec<&str>>();
        match toks[0] {
            "hlf" => {
                match toks[1] {
                    "a" => a /= 2,
                    "b" => b /= 2,
                    _ => panic!(),
                }
                ip += 1;
            }
            "tpl" => {
                match toks[1] {
                    "a" => a *= 3,
                    "b" => b *= 3,
                    _ => panic!(),
                }
                ip += 1;
            }
            "inc" => {
                match toks[1] {
                    "a" => a += 1,
                    "b" => b += 1,
                    _ => panic!(),
                }
                ip += 1;
            }
            "jmp" => {
                let offset = toks[1].parse::<i32>().unwrap();
                ip += offset;
            }
            "jie" => {
                let offset = toks[2].parse::<i32>().unwrap();
                match toks[1] {
                    "a," => ip += if a % 2 == 0 { offset } else { 1 },
                    "b," => ip += if b % 2 == 0 { offset } else { 1 },
                    _ => panic!(),
                }
            }
            "jio" => {
                let offset = toks[2].parse::<i32>().unwrap();
                match toks[1] {
                    "a," => ip += if a == 1 { offset } else { 1 },
                    "b," => ip += if b == 2 { offset } else { 1 },
                    _ => panic!(),
                }
            }
            _ => panic!("Invalid instruction {}", toks[0]),
        }
    }

    b
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input, 0));
    println!("{}", solve(input, 1));
}
