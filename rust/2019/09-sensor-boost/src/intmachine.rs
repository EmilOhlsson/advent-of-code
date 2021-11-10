pub enum IOState {
    Output(i64),
    Input,
    Done,
}

#[derive(Clone)]
pub struct Intmachine {
    output: Vec<i64>,
    program: Vec<i64>,
    trace: Vec<usize>,
    heatmap: Vec<usize>,
    ip: usize, // Instruction pointer
    rb: i64,   // Relative base
    debug: bool,
}

impl Intmachine {
    /// Read an operand parameter
    fn get(&self, pos: usize) -> i64 {
        let mut mode = self.program[self.ip] / 100;
        for _ in 1..pos {
            mode /= 10;
        }
        let opval = self.program[self.ip + pos];
        match mode % 10 {
            0 => self.program[opval as usize],             // Positional mode
            1 => opval,                                    // Immediate mode
            2 => self.program[(opval + self.rb) as usize], // Relative mode
            _ => panic!("Invalid mode :("),
        }
    }

    /// Set an operand parameter
    fn set(&mut self, pos: usize, val: i64) {
        let mut mode = self.program[self.ip] / 100;
        for _ in 1..pos {
            mode /= 10;
        }
        let opval = self.program[self.ip + pos];
        match mode % 10 {
            0 => self.program[opval as usize] = val, // Positional mode
            2 => self.program[(opval + self.rb) as usize] = val, // Relative mode
            _ => panic!("Invalid mode :("),
        }
    }

    /// Load program and create a new intmachine
    pub fn load(input: &str) -> Intmachine {
        let mut program = input
            .trim()
            .split(',')
            .map(str::parse::<i64>)
            .map(Result::unwrap)
            .collect::<Vec<i64>>();
        let program_length = program.len();
        program.resize(16 * 1024 * 1024, 0);
        Intmachine {
            output: Vec::new(),
            program,
            trace: Vec::new(),
            heatmap: vec![0; program_length],
            ip: 0,
            rb: 0,
            debug: false,
        }
    }

    /// Start tracing and record heatmap
    pub fn debug(&mut self) {
        self.debug = true;
    }

    /// Run machine until exit instruction
    pub fn run<'a>(&mut self, input: impl IntoIterator<Item = &'a i64>) -> Vec<i64> {
        let mut ii = input.into_iter();
        loop {
            let inp = *ii.next().unwrap_or(&0);
            if let IOState::Done = self.run_to_event(inp) {
                return self.output.clone();
            }
        }
    }

    /// Run machine until IO or exit
    pub fn run_to_event(&mut self, input: i64) -> IOState {
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
                    self.set(3, (self.get(1) < self.get(2)) as i64);
                    self.ip += 4;
                }
                8 => {
                    // Equals
                    self.set(3, (self.get(1) == self.get(2)) as i64);
                    self.ip += 4;
                }
                9 => {
                    // Set relative base
                    self.rb += self.get(1);
                    self.ip += 2;
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
            9 => "SRB",
            99 => "EXIT",
            _ => "(unknown/invalid)",
        }
        .to_string()
    }

    pub fn dump_heatmap(&self) {
        let maxwidth = std::cmp::max(*self.heatmap.iter().max().unwrap(), 100);
        for (i, count) in self.heatmap.iter().enumerate() {
            if *count > 0 {
                let width = (100 * *count) / maxwidth;
                println!(
                    "[{:>5}] {:>5} #{:<5}: {}",
                    i,
                    self.instruction_name(i),
                    *count,
                    std::iter::repeat('#').take(width).collect::<String>()
                );
            }
        }
    }
}
