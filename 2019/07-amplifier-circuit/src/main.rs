use coding_challenge_utils::sets;

pub enum IOState {
    Output(i32),
    Input,
    Done,
}

#[derive(Clone)]
pub struct Intmachine {
    output: Vec<i32>,
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
            output: Vec::new(),
            program: input
                .trim()
                .split(',')
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .collect::<Vec<i32>>(),
            ip: 0,
        }
    }

    pub fn run<'a>(&mut self, input: impl IntoIterator<Item = &'a i32>) -> Vec<i32> {
        let mut ii = input.into_iter();
        loop {
            let inp = *ii.next().unwrap_or(&0);
            if let IOState::Done = self.run_to_event(inp) {
                return self.output.clone();
            }
        }
    }

    pub fn run_to_event(&mut self, input: i32) -> IOState {
        let mut consumed: bool = false;
        loop {
            match self.program[self.ip] % 100 {
                1 => {
                    // Add
                    self.set(3, self.get(1) + self.get(2));
                    self.ip += 4;
                }
                2 => {
                    // Mul
                    self.set(3, self.get(1) * self.get(2));
                    self.ip += 4;
                }
                3 => {
                    // Read
                    if consumed {
                        return IOState::Input;
                    }
                    consumed = true;
                    self.set(1, input);
                    self.ip += 2;
                }
                4 => {
                    // Write
                    let output = self.get(1);
                    self.output.push(output);
                    self.ip += 2;
                    return IOState::Output(output);
                }
                5 => {
                    // Jump if true
                    if self.get(1) != 0 {
                        self.ip = self.get(2) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    // Jump if false
                    if self.get(1) == 0 {
                        self.ip = self.get(2) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    // Less than
                    self.set(3, (self.get(1) < self.get(2)) as i32);
                    self.ip += 4;
                }
                8 => {
                    // Equals
                    self.set(3, (self.get(1) == self.get(2)) as i32);
                    self.ip += 4;
                }
                99 => {
                    // Exit
                    return IOState::Done;
                }
                i => panic!("Unknown instruction: {}", i),
            }
        }
    }
}

fn try_sequence(program: &Intmachine, phase: &[i32]) -> i32 {
    let mut signal = 0;
    let mut amps = vec![program.clone(); 5];
    for i in 0..5 {
        amps[i].run_to_event(phase[i]);
    }
    loop {
        let mut done = false;
        for amp in &mut amps {
            match amp.run_to_event(signal) {
                IOState::Output(output) => signal = output,
                IOState::Input => (),
                IOState::Done => done = true,
            }
        }
        if done {
            return signal;
        }
    }
}

fn solve(input: &str) -> i32 {
    let program = Intmachine::load(input);
    let combinations = sets::permutations(&mut vec![0, 1, 2, 3, 4]);

    combinations
        .iter()
        .map(|phase| try_sequence(&program, phase))
        .max()
        .unwrap()
}

fn solve_v2(input: &str) -> i32 {
    let program = Intmachine::load(input);
    let combinations = sets::permutations(&mut vec![5, 6, 7, 8, 9]);

    combinations
        .iter()
        .map(|phase| try_sequence(&program, phase))
        .max()
        .unwrap()
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input));
    println!("{:?}", solve_v2(input));
}

#[test]
fn test0() {
    let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    assert_eq!(solve(input), 43210);
}

#[test]
fn test1() {
    let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
    assert_eq!(solve(input), 54321);
}

#[test]
fn test2() {
    let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
    assert_eq!(solve(input), 65210);
}

#[test]
fn test3() {
    let input =
        "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
    assert_eq!(solve_v2(input), 139629729);
}

#[test]
fn test4() {
    let input = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
    assert_eq!(solve_v2(input), 18216);
}
