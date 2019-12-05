fn get_param(program: &Vec<i32>, instp: usize, pos: usize) -> usize {
    let mut mode = program[instp] / 100;
    for _ in 1..pos {
        mode /= 10;
    }
    match mode % 10 {
        0 => program[instp + pos] as usize,
        1 => instp + pos,
        _ => panic!("Invalid mode :("),
    }
}

fn solve(input: &str, stdin: &Vec<i32>) -> i32 {
    let mut program = input
        .split(',')
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect::<Vec<i32>>();

    let mut instp = 0;
    let mut stdin_ptr = 0;
    let mut output = Vec::new();

    loop {
        instp += match program[instp] % 100 {
            1 => {
                let ops = (
                    get_param(&program, instp, 1),
                    get_param(&program, instp, 2),
                    get_param(&program, instp, 3),
                );
                program[ops.2] = program[ops.0] + program[ops.1];
                4
            }
            2 => {
                let ops = (
                    get_param(&program, instp, 1),
                    get_param(&program, instp, 2),
                    get_param(&program, instp, 3),
                );
                program[ops.2] = program[ops.0] * program[ops.1];
                4
            }
            3 => {
                let op = get_param(&program, instp, 1);
                program[op] = stdin[stdin_ptr];
                stdin_ptr += 1;
                2
            }
            4 => {
                let op = get_param(&program, instp, 1);
                output.push(program[op]);
                2
            }
            5 => {
                // Jump if true
                let ops = (get_param(&program, instp, 1), get_param(&program, instp, 2));
                if program[ops.0] != 0 {
                    instp = program[ops.1] as usize;
                    0
                } else {
                    3
                }
            }
            6 => {
                // Jump if false
                let ops = (get_param(&program, instp, 1), get_param(&program, instp, 2));
                if program[ops.0] == 0 {
                    instp = program[ops.1] as usize;
                    0
                } else {
                    3
                }
            }
            7 => {
                // Less than
                let ops = (
                    get_param(&program, instp, 1),
                    get_param(&program, instp, 2),
                    get_param(&program, instp, 3),
                );
                program[ops.2] = if program[ops.0] < program[ops.1] {
                    1
                } else {
                    0
                };
                4
            }
            8 => {
                // Equals
                let ops = (
                    get_param(&program, instp, 1),
                    get_param(&program, instp, 2),
                    get_param(&program, instp, 3),
                );
                program[ops.2] = if program[ops.0] == program[ops.1] {
                    1
                } else {
                    0
                };
                4
            }
            99 => {
                println!("output: {:?}", output);
                return *output.last().unwrap();
            }
            _ => panic!("Invalid opcode {} at {}", program[instp], instp),
        };
    }
}

fn main() {
    let input = include_str!("input").to_string();
    println!("{}", solve(input.trim(), &vec![1]));
    println!("{}", solve(input.trim(), &vec![5]));
}

#[test]
fn test_get_param() {
    let program = vec![1002, 42, 666, 8];
    assert_eq!(get_param(&program, 0, 1), 42);
    assert_eq!(get_param(&program, 0, 2), 2);
    assert_eq!(get_param(&program, 0, 3), 8);
}

#[test]
fn cmp_eq() {
    let program = "3,9,8,9,10,9,4,9,99,-1,8";
    assert_eq!(solve(&program, &vec![7]), 0);
    assert_eq!(solve(&program, &vec![8]), 1);
    assert_eq!(solve(&program, &vec![9]), 0);
}

#[test]
fn cmp_lt() {
    let program = "3,9,7,9,10,9,4,9,99,-1,8";
    assert_eq!(solve(&program, &vec![7]), 1);
    assert_eq!(solve(&program, &vec![8]), 0);
    assert_eq!(solve(&program, &vec![9]), 0);
}

#[test]
fn cmp_eq_immediate() {
    let program = "3,3,1108,-1,8,3,4,3,99";
    assert_eq!(solve(&program, &vec![7]), 0);
    assert_eq!(solve(&program, &vec![8]), 1);
    assert_eq!(solve(&program, &vec![9]), 0);
}

#[test]
fn cmp_lt_immediate() {
    let program = "3,3,1107,-1,8,3,4,3,99";
    assert_eq!(solve(&program, &vec![7]), 1);
    assert_eq!(solve(&program, &vec![8]), 0);
    assert_eq!(solve(&program, &vec![9]), 0);
}

#[test]
fn cmp_pos() {
    let program = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
    assert_eq!(solve(&program, &vec![0]), 0);
    assert_eq!(solve(&program, &vec![1]), 1);
    assert_eq!(solve(&program, &vec![9]), 1);
}

#[test]
fn cmp_imm() {
    let program = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
    assert_eq!(solve(&program, &vec![0]), 0);
    assert_eq!(solve(&program, &vec![1]), 1);
    assert_eq!(solve(&program, &vec![9]), 1);
}

#[test]
fn test_p2() {
    let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    assert_eq!(solve(&program, &vec![7]), 999);
    assert_eq!(solve(&program, &vec![8]), 1000);
    assert_eq!(solve(&program, &vec![9]), 1001);
}
