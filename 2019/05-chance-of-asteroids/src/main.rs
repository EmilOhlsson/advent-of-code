struct Intmachine {
    program: Vec<i32>,
    ip: usize,
}

impl Intmachine {
    fn get(&self, pos: usize) -> i32 {
        let mut mode = self.program[self.ip] / 100;
        for _ in 1..pos {
            mode /= 10;
        }
        match mode % 10 {
            0 => self.program[self.program[self.ip + pos] as usize],
            1 => self.program[self.ip + pos],
            _ => panic!("Invalid mode :("),
        }
    }

    fn set(&mut self, pos: usize, val: i32) {
        let i = self.program[self.ip + pos] as usize;
        self.program[i] = val
    }

    fn load(input: &str) -> Intmachine {
        Intmachine {
            program: input
                .trim()
                .split(',')
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .collect::<Vec<i32>>(),
            ip: 0,
        }
    }

    fn run<'a>(&mut self, input: impl IntoIterator<Item = &'a i32>) -> Vec<i32> {
        let mut ii = input.into_iter();
        let mut output = Vec::new();
        loop {
            self.ip += match self.program[self.ip] % 100 {
                1 => {
                    // Add
                    self.set(3, self.get(1) + self.get(2));
                    4
                }
                2 => {
                    // Mul
                    self.set(3, self.get(1) * self.get(2));
                    4
                }
                3 => {
                    // Read
                    self.set(1, *ii.next().unwrap());
                    2
                }
                4 => {
                    // Write
                    output.push(self.get(1));
                    2
                }
                5 => {
                    // Jump if true
                    if self.get(1) != 0 {
                        self.ip = self.get(2) as usize;
                        0
                    } else {
                        3
                    }
                }
                6 => {
                    // Jump if false
                    if self.get(1) == 0 {
                        self.ip = self.get(2) as usize;
                        0
                    } else {
                        3
                    }
                }
                7 => {
                    // Less than
                    self.set(3, (self.get(1) < self.get(2)) as i32);
                    4
                }
                8 => {
                    // Equals
                    self.set(3, (self.get(1) == self.get(2)) as i32);
                    4
                }
                99 => {
                    // Exit
                    return output;
                }
                i @ _ => panic!("Unknown instruction: {}", i),
            }
        }
    }
}

fn solve(program: &str, input: &[i32]) -> i32 {
    let mut machine = Intmachine::load(program);
    let output = machine.run(input.iter());
    *output.last().unwrap()
}

fn main() {
    let input = include_str!("input").to_string();
    println!("{}", solve(input.trim(), &[1]));
    println!("{}", solve(input.trim(), &[5]));
}

#[test]
fn test_get() {
    let machine = Intmachine::load("1002,0,3,4,666");
    assert_eq!(machine.get(1), 1002);
    assert_eq!(machine.get(2), 3);
    assert_eq!(machine.get(3), 666);
}

#[test]
fn cmp_eq() {
    let program = "3,9,8,9,10,9,4,9,99,-1,8";
    assert_eq!(solve(&program, &[7]), 0);
    assert_eq!(solve(&program, &[8]), 1);
    assert_eq!(solve(&program, &[9]), 0);
}

#[test]
fn cmp_lt() {
    let program = "3,9,7,9,10,9,4,9,99,-1,8";
    assert_eq!(solve(&program, &[7]), 1);
    assert_eq!(solve(&program, &[8]), 0);
    assert_eq!(solve(&program, &[9]), 0);
}

#[test]
fn cmp_eq_immediate() {
    let program = "3,3,1108,-1,8,3,4,3,99";
    assert_eq!(solve(&program, &[7]), 0);
    assert_eq!(solve(&program, &[8]), 1);
    assert_eq!(solve(&program, &[9]), 0);
}

#[test]
fn cmp_lt_immediate() {
    let program = "3,3,1107,-1,8,3,4,3,99";
    assert_eq!(solve(&program, &[7]), 1);
    assert_eq!(solve(&program, &[8]), 0);
    assert_eq!(solve(&program, &[9]), 0);
}

#[test]
fn cmp_pos() {
    let program = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
    assert_eq!(solve(&program, &[0]), 0);
    assert_eq!(solve(&program, &[1]), 1);
    assert_eq!(solve(&program, &[9]), 1);
}

#[test]
fn cmp_imm() {
    let program = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
    assert_eq!(solve(&program, &[0]), 0);
    assert_eq!(solve(&program, &[1]), 1);
    assert_eq!(solve(&program, &[9]), 1);
}

#[test]
fn test_p2() {
    let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    assert_eq!(solve(&program, &[7]), 999);
    assert_eq!(solve(&program, &[8]), 1000);
    assert_eq!(solve(&program, &[9]), 1001);
}
