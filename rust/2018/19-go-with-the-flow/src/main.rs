use std::collections::HashMap;

fn addr(regs: &mut Vec<usize>, a: usize, b: usize, c: usize) -> Result<(), ()> {
    regs[c] = regs.get(a).ok_or(())? + regs.get(b).ok_or(())?;
    Ok(())
}

fn addi(regs: &mut Vec<usize>, a: usize, b: usize, c: usize) -> Result<(), ()> {
    regs[c] = regs.get(a).ok_or(())? + b;
    Ok(())
}

fn mulr(regs: &mut Vec<usize>, a: usize, b: usize, c: usize) -> Result<(), ()> {
    regs[c] = regs.get(a).ok_or(())? * regs.get(b).ok_or(())?;
    Ok(())
}

fn muli(regs: &mut Vec<usize>, a: usize, b: usize, c: usize) -> Result<(), ()> {
    regs[c] = regs.get(a).ok_or(())? * b;
    Ok(())
}

fn banr(regs: &mut Vec<usize>, a: usize, b: usize, c: usize) -> Result<(), ()> {
    regs[c] = regs.get(a).ok_or(())? & regs.get(b).ok_or(())?;
    Ok(())
}

fn bani(regs: &mut Vec<usize>, a: usize, b: usize, c: usize) -> Result<(), ()> {
    regs[c] = regs.get(a).ok_or(())? & b;
    Ok(())
}

fn borr(regs: &mut Vec<usize>, a: usize, b: usize, c: usize) -> Result<(), ()> {
    regs[c] = regs.get(a).ok_or(())? | regs.get(b).ok_or(())?;
    Ok(())
}

fn bori(regs: &mut Vec<usize>, a: usize, b: usize, c: usize) -> Result<(), ()> {
    regs[c] = regs.get(a).ok_or(())? | b;
    Ok(())
}

fn setr(regs: &mut Vec<usize>, a: usize, _: usize, c: usize) -> Result<(), ()> {
    regs[c] = *regs.get(a).ok_or(())?;
    Ok(())
}

fn seti(regs: &mut Vec<usize>, a: usize, _: usize, c: usize) -> Result<(), ()> {
    regs[c] = a;
    Ok(())
}

fn gtir(regs: &mut Vec<usize>, a: usize, b: usize, c: usize) -> Result<(), ()> {
    regs[c] = if a > *regs.get(b).ok_or(())? { 1 } else { 0 };
    Ok(())
}

fn gtri(regs: &mut Vec<usize>, a: usize, b: usize, c: usize) -> Result<(), ()> {
    regs[c] = if *regs.get(a).ok_or(())? > b { 1 } else { 0 };
    Ok(())
}

fn gtrr(regs: &mut Vec<usize>, a: usize, b: usize, c: usize) -> Result<(), ()> {
    regs[c] = if regs.get(a).ok_or(())? > regs.get(b).ok_or(())? {
        1
    } else {
        0
    };
    Ok(())
}

fn eqir(regs: &mut Vec<usize>, a: usize, b: usize, c: usize) -> Result<(), ()> {
    regs[c] = if a == *regs.get(b).ok_or(())? { 1 } else { 0 };
    Ok(())
}

fn eqri(regs: &mut Vec<usize>, a: usize, b: usize, c: usize) -> Result<(), ()> {
    regs[c] = if *regs.get(a).ok_or(())? == b { 1 } else { 0 };
    Ok(())
}

fn eqrr(regs: &mut Vec<usize>, a: usize, b: usize, c: usize) -> Result<(), ()> {
    regs[c] = if regs.get(a).ok_or(())? == regs.get(b).ok_or(())? {
        1
    } else {
        0
    };
    Ok(())
}

fn solve_p1(input: &str, ip_reg: usize) -> usize {
    let mut isa: HashMap<String, Box<Fn(&mut Vec<usize>, usize, usize, usize) -> Result<(), ()>>> =
        HashMap::new();
    isa.insert("addr".to_owned(), Box::new(addr));
    isa.insert("addi".to_owned(), Box::new(addi));
    isa.insert("mulr".to_owned(), Box::new(mulr));
    isa.insert("muli".to_owned(), Box::new(muli));
    isa.insert("banr".to_owned(), Box::new(banr));
    isa.insert("bani".to_owned(), Box::new(bani));
    isa.insert("borr".to_owned(), Box::new(borr));
    isa.insert("bori".to_owned(), Box::new(bori));
    isa.insert("seti".to_owned(), Box::new(seti));
    isa.insert("setr".to_owned(), Box::new(setr));
    isa.insert("gtir".to_owned(), Box::new(gtir));
    isa.insert("gtri".to_owned(), Box::new(gtri));
    isa.insert("gtrr".to_owned(), Box::new(gtrr));
    isa.insert("eqir".to_owned(), Box::new(eqir));
    isa.insert("eqrr".to_owned(), Box::new(eqrr));
    isa.insert("eqri".to_owned(), Box::new(eqri));

    let program: Vec<(String, usize, usize, usize)> = input
        .lines()
        .map(|l| {
            let toks = l.split_whitespace().collect::<Vec<&str>>();
            (
                toks[0].to_owned(),
                toks[1].parse::<usize>().unwrap(),
                toks[2].parse::<usize>().unwrap(),
                toks[3].parse::<usize>().unwrap(),
            )
        })
        .collect();

    let mut registers = vec![0; 6];
    registers[0] = 1;

    loop {
        let ip = registers[ip_reg];
        if let Some((inst, a, b, c)) = program.get(ip) {
            //println!("ip={} {:?} -- {} {} {} {}", ip, registers, inst, a, b, c);
            let instr = isa.get(inst).unwrap();
            instr(&mut registers, *a, *b, *c).unwrap();
        } else {
            return registers[0];
        }
        registers[ip_reg] += 1;
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", solve_p1(input, 2));
}

#[test]
fn test() {
    let input = include_str!("input-simple.txt");
    assert_eq!(solve_p1(input, 0), 7);
}
