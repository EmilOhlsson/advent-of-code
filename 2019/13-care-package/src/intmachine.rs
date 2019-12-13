use std::collections::HashMap;

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
    length: usize,
    ip: usize, // Instruction pointer
    rb: i64,   // Relative base
    debug: bool,
}

const ADD: i64 = 1;
const MUL: i64 = 2;
const READ: i64 = 3;
const WRITE: i64 = 4;
const JNZ: i64 = 5;
const JZ: i64 = 6;
const LT: i64 = 7;
const EQ: i64 = 8;
const SETRB: i64 = 9;
const EXIT: i64 = 99;

enum Mode {
    Pos,
    Imm,
    Rb,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Mode::Pos => "o",
                Mode::Imm => "#",
                Mode::Rb => "r",
            }
        )
    }
}

impl Intmachine {
    fn getm(&self, ip: usize, pos: usize) -> Mode {
        let mut mode = self.program[ip] / 100;
        for _ in 1..pos {
            mode /= 10;
        }

        match mode % 10 {
            0 => Mode::Pos,
            1 => Mode::Imm,
            2 => Mode::Rb,
            m => panic!("Invalid mode: {}", m),
        }
    }

    fn getv(&self, ip: usize, pos: usize) -> i64 {
        let opval = self.program[ip + pos];
        match self.getm(ip, pos) {
            Mode::Pos => self.program[opval as usize],
            Mode::Imm => opval,
            Mode::Rb => self.program[(opval + self.rb) as usize],
        }
    }

    /// Read an operand parameter
    fn get(&self, pos: usize) -> i64 {
        self.getv(self.ip, pos)
    }

    /// Set an operand parameter
    fn set(&mut self, pos: usize, val: i64) {
        let opval = self.program[self.ip + pos];
        match self.getm(self.ip, pos) {
            Mode::Pos => self.program[opval as usize] = val, // Positional mode
            Mode::Imm => panic!("Write to immediate parameter"),
            Mode::Rb => self.program[(opval + self.rb) as usize] = val, // Relative mode
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
            length: program_length,
            ip: 0,
            rb: 0,
            debug: false,
        }
    }

    /// Set value at specific address
    pub fn set_addr(&mut self, addr: usize, value: i64) {
        self.program[addr] = value;
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
            if let IOState::Done = self.run_to_event(Some(inp)) {
                return self.output.clone();
            }
        }
    }

    /// Run machine until IO or exit
    pub fn run_to_event(&mut self, input: Option<i64>) -> IOState {
        loop {
            if let Some(event) = self.run_instruction(input) {
                match event {
                    IOState::Done => return IOState::Done,
                    IOState::Input => return IOState::Input,
                    IOState::Output(v) => return IOState::Output(v),
                }
            }
        }
    }

    fn run_instruction(&mut self, input: Option<i64>) -> Option<IOState> {
        if self.debug {
            self.heatmap[self.ip] += 1;
            self.trace.push(self.ip);
        }

        match self.program[self.ip] % 100 {
            ADD => {
                self.set(3, self.get(1) + self.get(2));
                self.ip += 4;
                return None;
            }
            MUL => {
                self.set(3, self.get(1) * self.get(2));
                self.ip += 4;
                return None;
            }
            READ => {
                self.set(1, input.unwrap());
                self.ip += 2;
                return Some(IOState::Input);
            }
            WRITE => {
                let output = self.get(1);
                self.output.push(output);
                self.ip += 2;
                return Some(IOState::Output(output));
            }
            JNZ => {
                if self.get(1) != 0 {
                    self.ip = self.get(2) as usize;
                } else {
                    self.ip += 3;
                }

                return None;
            }
            JZ => {
                if self.get(1) == 0 {
                    self.ip = self.get(2) as usize;
                } else {
                    self.ip += 3;
                }
                return None;
            }
            LT => {
                self.set(3, (self.get(1) < self.get(2)) as i64);
                self.ip += 4;
                return None;
            }
            EQ => {
                self.set(3, (self.get(1) == self.get(2)) as i64);
                self.ip += 4;
                return None;
            }
            SETRB => {
                self.rb += self.get(1);
                self.ip += 2;
                return None;
            }
            EXIT => {
                return Some(IOState::Done);
            }
            i => panic!("Unknown instruction: {}", i),
        }
    }

    pub fn trace(&self) -> Vec<usize> {
        self.trace.clone()
    }

    fn instruction_name(&self, instruction_pointer: usize) -> String {
        match self.program[instruction_pointer] % 100 {
            ADD => "ADD",
            MUL => "MUL",
            READ => "READ",
            WRITE => "WRITE",
            JNZ => "JNZ",
            JZ => "JZ",
            LT => "LT",
            EQ => "EQ",
            SETRB => "SETRB",
            EXIT => "EXIT",
            _ => "(unknown/invalid)",
        }
        .to_string()
    }

    fn getp(&self, ip: usize, pos: usize) -> String {
        format!("{}{}", self.getm(ip, pos), self.program[ip + pos])
    }

    fn disassemble_addr(&self, ip: usize) -> (usize, String, Option<usize>) {
        match self.program[ip] % 100 {
            ADD => (
                4,
                format!(
                    "ADD {} + {} -> {}",
                    self.getp(ip, 1),
                    self.getp(ip, 2),
                    self.getp(ip, 3)
                ),
                None,
            ),
            MUL => (
                4,
                format!(
                    "MUL {} * {} -> {}",
                    self.getp(ip, 1),
                    self.getp(ip, 2),
                    self.getp(ip, 3)
                ),
                None,
            ),
            READ => (2, format!("READ -> {}", self.getp(ip, 1),), None),
            WRITE => (2, format!("WRITE {}", self.getp(ip, 1),), None),
            JNZ => (
                3,
                format!("JNZ {} to {}", self.getp(ip, 1), self.getp(ip, 2)),
                Some(self.getv(ip, 2) as usize),
            ),
            JZ => (
                3,
                format!("JZ {} to {}", self.getp(ip, 1), self.getp(ip, 2)),
                Some(self.getv(ip, 2) as usize),
            ),
            LT => (
                4,
                format!(
                    "LT {} < {} -> {}",
                    self.getp(ip, 1),
                    self.getp(ip, 2),
                    self.getp(ip, 3)
                ),
                None,
            ),
            EQ => (
                4,
                format!(
                    "EQ {} == {} -> {}",
                    self.getp(ip, 1),
                    self.getp(ip, 2),
                    self.getp(ip, 3)
                ),
                None,
            ),
            SETRB => (2, format!("SETRB {}", self.getp(ip, 1)), None),
            EXIT => (1, "EXIT".to_string(), None),
            _ => (1, format!("{}", self.program[ip]), None),
        }
    }

    /// Attemp to create something readble-ish
    pub fn disassemble(&self, use_heatmap: bool) {
        let mut ip = 0;
        let mut ips = Vec::new();
        let mut dests = HashMap::new();
        let mut instructions = HashMap::new();
        while ip < self.length {
            if (use_heatmap && self.heatmap[ip] > 0) || !use_heatmap {
                let (size, rep, jmp) = self.disassemble_addr(ip);
                ips.push(ip);
                instructions.insert(ip, rep);
                if let Some(dst) = jmp {
                    dests.insert(dst, ip);
                }
                ip += size;
            } else {
                ip += 1;
            }
        }

        for i in &ips {
            if let Some(from) = dests.get(i) {
                println!("From {}:", from);
            }
            println!("  {} -- {}", i, instructions[i]);
        }
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
