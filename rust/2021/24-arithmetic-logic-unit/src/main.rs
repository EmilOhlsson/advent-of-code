#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Reg {
    W,
    X,
    Y,
    Z,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
struct Alu {
    regs: [i64; 4],
}

impl Alu {
    fn new() -> Alu {
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

        let mut ids = [0, 0, 0, 0];
        let mut input_id = 0;

        println!("digraph {{");
        for instr in prog {
            match instr {
                Inp(dst) => {
                    self.regs[dst.index()] = *vals.next().unwrap();

                    ids[dst.index()] += 1;
                    println!(
                        "\"{}_{}\" [label = \"input-{} -> {}\" ]",
                        dst.to(),
                        ids[dst.index()],
                        input_id,
                        dst.to()
                    );
                    input_id += 1;
                }
                Add(dst, src) => {
                    let prev = self.get(*dst);
                    self.set(*dst, prev + self.val(*src));

                    println!(
                        "\"{}_{}\" -> \"{}_{}\" [label = \"{}={}\" ]",
                        dst.to(),
                        ids[dst.index()],
                        dst.to(),
                        ids[dst.index()] + 1,
                        dst.to(),
                        prev
                    );
                    match src {
                        Val::Imm(v) => {
                            println!(
                                "\"{}_{}\" [label = \"{} += {}\"]",
                                dst.to(),
                                ids[dst.index()] + 1,
                                dst.to(),
                                v
                            );
                        }
                        Val::Reg(r) => {
                            println!(
                                "\"{}_{}\" [label = \"{} += {}\"]",
                                dst.to(),
                                ids[dst.index()] + 1,
                                dst.to(),
                                r.to()
                            );
                            println!(
                                "\"{}_{}\" -> \"{}_{}\" [label =\"{}={}\"]",
                                r.to(),
                                ids[r.index()],
                                dst.to(),
                                ids[dst.index()] + 1,
                                r.to(),
                                self.get(*r)
                            );
                        }
                    }
                    ids[dst.index()] += 1;
                }
                Mul(dst, src) => {
                    let prev = self.get(*dst);
                    self.set(*dst, prev * self.val(*src));

                    println!(
                        "\"{}_{}\" -> \"{}_{}\" [label = \"{}={}\" ]",
                        dst.to(),
                        ids[dst.index()],
                        dst.to(),
                        ids[dst.index()] + 1,
                        dst.to(),
                        prev
                    );
                    match src {
                        Val::Imm(v) => {
                            println!(
                                "\"{}_{}\" [label = \"{} *= {}\"]",
                                dst.to(),
                                ids[dst.index()] + 1,
                                dst.to(),
                                v
                            );
                        }
                        Val::Reg(r) => {
                            println!(
                                "\"{}_{}\" [label = \"{} *= {}\"]",
                                dst.to(),
                                ids[dst.index()] + 1,
                                dst.to(),
                                r.to()
                            );
                            println!(
                                "\"{}_{}\" -> \"{}_{}\" [label =\"{}={}\"]",
                                r.to(),
                                ids[r.index()],
                                dst.to(),
                                ids[dst.index()] + 1,
                                r.to(),
                                self.get(*r)
                            );
                        }
                    }
                    ids[dst.index()] += 1;
                }
                Div(dst, src) => {
                    let prev = self.get(*dst);
                    self.set(*dst, prev / self.val(*src));

                    println!(
                        "\"{}_{}\" -> \"{}_{}\" [label = \"{}={}\" ]",
                        dst.to(),
                        ids[dst.index()],
                        dst.to(),
                        ids[dst.index()] + 1,
                        dst.to(),
                        prev
                    );
                    match src {
                        Val::Imm(v) => {
                            println!(
                                "\"{}_{}\" [label = \"{} /= {}\"]",
                                dst.to(),
                                ids[dst.index()] + 1,
                                dst.to(),
                                v
                            );
                        }
                        Val::Reg(r) => {
                            println!(
                                "\"{}_{}\" [label = \"{} /= {}\"]",
                                dst.to(),
                                ids[dst.index()] + 1,
                                dst.to(),
                                r.to()
                            );
                            println!(
                                "\"{}_{}\" -> \"{}_{}\" [label =\"{}={}\"]",
                                r.to(),
                                ids[r.index()],
                                dst.to(),
                                ids[dst.index()] + 1,
                                r.to(),
                                self.get(*r)
                            );
                        }
                    }
                    ids[dst.index()] += 1;
                }
                Mod(dst, src) => {
                    let prev = self.get(*dst);
                    self.set(*dst, prev % self.val(*src));

                    println!(
                        "\"{}_{}\" -> \"{}_{}\" [label = \"{}={}\" ]",
                        dst.to(),
                        ids[dst.index()],
                        dst.to(),
                        ids[dst.index()] + 1,
                        dst.to(),
                        prev
                    );
                    match src {
                        Val::Imm(v) => {
                            println!(
                                "\"{}_{}\" [label = \"{} %= {}\"]",
                                dst.to(),
                                ids[dst.index()] + 1,
                                dst.to(),
                                v
                            );
                        }
                        Val::Reg(r) => {
                            println!(
                                "\"{}_{}\" [label = \"{} %= {}\"]",
                                dst.to(),
                                ids[dst.index()] + 1,
                                dst.to(),
                                r.to()
                            );
                            println!(
                                "\"{}_{}\" -> \"{}_{}\" [label =\"{}={}\"]",
                                r.to(),
                                ids[r.index()],
                                dst.to(),
                                ids[dst.index()] + 1,
                                r.to(),
                                self.get(*r)
                            );
                        }
                    }
                    ids[dst.index()] += 1;
                }
                Eql(dst, src) => {
                    let prev = self.get(*dst);
                    self.set(*dst, (prev == self.val(*src)) as i64);

                    println!(
                        "\"{}_{}\" -> \"{}_{}\" [label = \"{}={}\" ]",
                        dst.to(),
                        ids[dst.index()],
                        dst.to(),
                        ids[dst.index()] + 1,
                        dst.to(),
                        prev
                    );
                    match src {
                        Val::Imm(v) => {
                            println!(
                                "\"{}_{}\" [label = \"{} = {} == {}\"]",
                                dst.to(),
                                ids[dst.index()] + 1,
                                dst.to(),
                                dst.to(),
                                v
                            );
                        }
                        Val::Reg(r) => {
                            println!(
                                "\"{}_{}\" [label = \"{} = {} == {}\"]",
                                dst.to(),
                                ids[dst.index()] + 1,
                                dst.to(),
                                dst.to(),
                                r.to()
                            );
                            println!(
                                "\"{}_{}\" -> \"{}_{}\" [label =\"{}={}\"]",
                                r.to(),
                                ids[r.index()],
                                dst.to(),
                                ids[dst.index()] + 1,
                                r.to(),
                                self.get(*r)
                            );
                        }
                    }
                    ids[dst.index()] += 1;
                }
                Clr(dst) => {
                    self.set(*dst, 0);

                    ids[dst.index()] += 1;
                    println!(
                        "\"{}_{}\" [label = \"{}=0\"]",
                        dst.to(),
                        ids[dst.index()],
                        dst.to()
                    );
                }
            }
        }
        println!("}}");
    }
}

fn parse(input: &str) -> Vec<Instr> {
    input
        .lines()
        .map(|line| {
            let mut tokens = line.split_whitespace();
            let instr = tokens.next().unwrap();
            let dst = Reg::from(tokens.next().unwrap());
            match instr {
                "inp" => Instr::Inp(dst),
                "add" => {
                    let val = Val::from(tokens.next().unwrap());
                    Instr::Add(dst, val)
                }
                "mul" => {
                    let val = Val::from(tokens.next().unwrap());
                    if val == Val::Imm(0) {
                        Instr::Clr(dst)
                    } else {
                        Instr::Mul(dst, val)
                    }
                }
                "div" => {
                    let val = Val::from(tokens.next().unwrap());
                    Instr::Div(dst, val)
                }
                "mod" => {
                    let val = Val::from(tokens.next().unwrap());
                    Instr::Mod(dst, val)
                }
                "eql" => {
                    let val = Val::from(tokens.next().unwrap());
                    Instr::Eql(dst, val)
                }
                _ => panic!("No such instruction"),
            }
        })
        .collect()
}

fn main() {
    let input = include_str!("input");
    let program = parse(input);

    let mut alu = Alu::new();
    //alu.run(&program, &[9, 9, 2, 9, 9, 5, 1, 3, 8, 9, 9, 9, 7, 1]);
    alu.run(&program, &[9, 3, 1, 8, 5, 1, 1, 1, 1, 2, 7, 9, 1, 1]);
}
