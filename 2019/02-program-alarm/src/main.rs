fn solve(input: &str, param: (usize, usize)) -> usize {
    let mut program = input
        .split(',')
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect::<Vec<usize>>();

    program[1] = param.0;
    program[2] = param.1;

    let mut instp = 0;
    loop {
        instp += match program[instp] {
            1 => {
                let ops = (program[instp + 1], program[instp + 2], program[instp + 3]);
                program[ops.2] = program[ops.0] + program[ops.1];
                4
            }
            2 => {
                let ops = (program[instp + 1], program[instp + 2], program[instp + 3]);
                program[ops.2] = program[ops.0] * program[ops.1];
                4
            }
            99 => return program[0],
            _ => panic!("Invalid opcode {} at {}", program[instp], instp),
        };
    }
}

fn main() {
    let input = include_str!("input").to_string();
    let trimmed = input.trim();
    println!("{}", solve(&trimmed, (12, 02)));

    let limit = 121;
    for noun in 0..limit {
        for verb in 0..limit {
            if solve(&trimmed, (noun, verb)) == 19_690_720 {
                println!("{}", 100 * noun + verb);
                return;
            }
        }
    }
}
