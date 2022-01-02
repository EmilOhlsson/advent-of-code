#[derive(Clone,Copy,Debug)]
enum Reg {
    W, X, Y, Z,
}

impl Reg {
    fn index(&self) -> usize {
        match self {
            Reg::W => 0,
            Reg::X => 1,
            Reg::Y => 2,
            Reg::Z => 3,
        }
    }
    fn from(s: &str) -> Reg {
        match s {
            "w" => Reg::W,
            "x" => Reg::X,
            "y" => Reg::Y,
            "z" => Reg::Z,
            _ => panic!(),
        }
    }
    fn to(&self) -> char {
        match self {
            Reg::W => 'W',
            Reg::X => 'X',
            Reg::Y => 'Y',
            Reg::Z => 'Z',
        }
    }
}

#[derive(Clone,Copy,Debug)]
enum Val {
    Reg(Reg),
    Imm(i64),
}

impl Val {
    fn from(s: &str) -> Val {
        if let Ok(num) = s.parse::<i64>() {
            Val::Imm(num)
        } else {
            Val::Reg(Reg::from(s))
        }
    }
}

enum Instr {
    Inp(Reg),
    Add(Reg, Val),
    Mul(Reg, Val),
    Div(Reg, Val),
    Mod(Reg, Val),
    Eql(Reg, Val),
    
    /* Dummy instruction for culling */
    Clr(Reg),
}

#[derive(Debug, Default)]
struct ALU {
    regs: [i64;4]
}

impl ALU {
    fn new() -> ALU {
        Default::default()
    }

    fn get(&self, reg: Reg) -> i64 {
        self.regs[reg.index()]
    }

    fn set(&mut self, reg: Reg, val: i64) {
        self.regs[reg.index()] = val;
    }

    fn val(&self, val: Val) -> i64 {
        match val {
            Val::Reg(r) => self.get(r),
            Val::Imm(v) => v,
        } 
    }

    fn run(&mut self, prog: &[Instr], input: &[i64]) {
        use Instr::*;
        let mut vals = input.iter();
        for instr in prog {
            match instr {
                Inp(dst) => self.regs[dst.index()] = *vals.next().unwrap(),
                Add(dst, src) => self.set(*dst, self.get(*dst) + self.val(*src)),
                Mul(dst, src) => self.set(*dst, self.get(*dst) * self.val(*src)),
                Div(dst, src) => self.set(*dst, self.get(*dst) / self.val(*src)),
                Mod(dst, src) => self.set(*dst, self.get(*dst) % self.val(*src)),
                Eql(dst, src) => self.set(*dst, (self.get(*dst) == self.val(*src)) as i64),
                Clr(dst) => self.set(*dst, 0),
            }
        }
    }

}

fn parse(input: &str) -> Vec<Instr> {
    input.lines().map(|line| {
        let mut tokens = line.split_whitespace();
        let instr = tokens.next().unwrap();
        let dst = Reg::from(tokens.next().unwrap());
        match instr {
            "inp" => {
                Instr::Inp(dst)
            },
            "add" => {
                let val = Val::from(tokens.next().unwrap());
                Instr::Add(dst, val)
            },
            "mul" => {
                let val = Val::from(tokens.next().unwrap());
                Instr::Mul(dst, val)
            },
            "div" => {
                let val = Val::from(tokens.next().unwrap());
                Instr::Div(dst, val)
            },
            "mod" => {
                let val = Val::from(tokens.next().unwrap());
                Instr::Mod(dst, val)
            },
            "eql" => {
                let val = Val::from(tokens.next().unwrap());
                Instr::Eql(dst, val)
            },
            _ => panic!("No such instruction")
        }
    }).collect()
}

fn brute_force(input: &str) -> i64{
    let program = parse(input);
    let mut nums: [i64;14] = [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9];
    loop {
        let mut alu = ALU::new();
        //println!("Checking: {:?}", nums);
        alu.run(&program, &nums);
        if alu.get(Reg::Z) == 0 {
            let mut res = 0;
            for n in &nums {
                res *= 10;
                res += n;
            }
            return res;
        } else {
            for n in nums.iter_mut().rev() {
                *n -= 1;
                if *n != 0 { break; }
                else {
                    *n = 9;
                }
            }
        }
    }
}

fn generate_op(ids: &mut [i64;4], id: &mut i64, op: &str, dst: Reg, src: Val) {
    let op_id = *id;
    println!("\"{}_{}\" [label = \"{}\"]", op, id,op);
    println!("\"{}_{}\" [label = \"{}\"]", dst.to(), ids[dst.index()], dst.to());
    *id += 1;

    /* Link source value to operator */
    match src {
        Val::Imm(v) => {
            /* create "unique number", and link to operator node */
            println!("\"{}_{}\" [label = \"{}\"]", v, id, v);
            println!("\"{}_{}\" -> \"{}_{}\"", v, id, op, op_id);
            *id += 1;
        },
        Val::Reg(r) => {
            println!("\"{}_{}\" [label = \"{}\"]", r.to(), ids[r.index()], r.to());
            println!("\"{}_{}\" -> \"{}_{}\"", r.to(), ids[r.index()], op, op_id);
        }
    }

    /* Link op to dst */
    println!("\"{}_{}\" -> \"{}_{}\"", op, op_id, dst.to(), ids[dst.index()]);

    /* Link prev dst to source */
    ids[dst.index()] += 1;
    println!("\"{}_{}\" -> \"{}_{}\"", dst.to(), ids[dst.index()], op, op_id);
}

fn generate_graph(instructions: &[Instr]) {
    use Instr::*;
    println!("digraph {{");
    //println!("A -> {{B C}}");
    let mut ids = [0, 0, 0, 0];
    let mut id = 0;
    let mut inp_id = 0;


    for instr in instructions.iter().rev() {
        match instr {
            Inp(dst) => {
                println!("\"{}_{}\" [label = \"{}\"]", dst.to(), ids[dst.index()], dst.to());
                //println!("\"input_{}\" [label = \"Inp\"]", inp);
                println!("\"input_{}\" -> \"{}_{}\"", inp_id, dst.to(), ids[dst.index()]);
                inp_id += 1;
                ids[dst.index()] += 1;
            }
            Clr(dst) => {
                println!("\"0_{}\" [label = \"0\"]", id);
                println!("\"{}_{}\" [label = \"{}\"]", dst.to(), ids[dst.index()], dst.to());
                println!("\"0_{}\" -> \"{}_{}\"", id, dst.to(), ids[dst.index()]);
                id += 1;
                ids[dst.index()] += 1;
            }
            Add(dst, src) => generate_op(&mut ids, &mut id, "+", *dst, *src),
            Mul(dst, src) => generate_op(&mut ids, &mut id, "*", *dst, *src),
            Div(dst, src) => generate_op(&mut ids, &mut id, "/", *dst, *src),
            Mod(dst, src) => generate_op(&mut ids, &mut id, "%", *dst, *src),
            Eql(dst, src) => generate_op(&mut ids, &mut id, "==", *dst, *src),
        }
    }
    println!("}}");
}

fn cull_constants(prog: &mut [Instr]) {
    for instr in prog {
        if let Instr::Mul(dst, Val::Imm(0)) = instr {
            *instr = Instr::Clr(*dst);
        }
    }
}

fn main() {
    let input = include_str!("input");
    let mut program = parse(&input);
    cull_constants(&mut program);
    generate_graph(&program);
    //println!("{}", brute_force(input));
}
