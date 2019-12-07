pub enum IOState {
    Output(i32),
    Input,
    Done,
}

#[derive(Clone)]
pub struct Intmachine {
    output: Vec<i32>,
    program: Vec<i32>,
    trace: Vec<usize>,
    heatmap: Vec<usize>,
    ip: usize,
    debug: bool,
}

impl Intmachine {
    /// Read an operand parameter
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

    /// Set an operand parameter
    fn set(&mut self, pos: usize, val: i32) {
        let i = self.program[self.ip + pos] as usize;
        self.program[i] = val
    }

    /// Load program and create a new intmachine
    pub fn load(input: &str) -> Intmachine {
        let program = input
            .trim()
            .split(',')
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .collect::<Vec<i32>>();
        let program_length = program.len();
        Intmachine {
            output: Vec::new(),
            program,
            trace: Vec::new(),
            heatmap: vec![0; program_length],
            ip: 0,
            debug: false,
        }
    }

    /// Start tracing and record heatmap
    pub fn debug(&mut self) {
        self.debug = true;
    }

    /// Run machine until exit instruction
    pub fn run<'a>(&mut self, input: impl IntoIterator<Item = &'a i32>) -> Vec<i32> {
        let mut ii = input.into_iter();
        loop {
            let inp = *ii.next().unwrap_or(&0);
            if let IOState::Done = self.run_to_event(inp) {
                return self.output.clone();
            }
        }
    }

    /// Run machine until IO or exit
    pub fn run_to_event(&mut self, input: i32) -> IOState {
        let mut consumed: bool = false;
        loop {
            if self.debug {
                self.heatmap[self.ip] += 1;
                self.trace.push(self.ip);
            }
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

    pub fn trace(&self) -> Vec<usize> {
        self.trace.clone()
    }

    fn instruction_name(&self, instruction_pointer: usize) -> String {
        match self.program[instruction_pointer] % 100 {
            1 => "ADD",
            2 => "MUL",
            3 => "READ",
            4 => "WRITE",
            5 => "JNZ",
            6 => "JZ",
            7 => "LT",
            8 => "EQ",
            99 => "EXIT",
            _ => "(unknown/invalid)",
        }
        .to_string()
    }

    pub fn dump_heatmap(&self) {
        for (i, count) in self.heatmap.iter().enumerate() {
            if *count > 0 {
                println!(
                    "[{:>5}] {:>5} #{:<5}: {:#4$}",
                    i,
                    self.instruction_name(i),
                    *count,
                    '#',
                    *count
                );
            }
        }
    }
}
