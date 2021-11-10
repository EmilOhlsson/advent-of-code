use std::collections::{HashMap, HashSet};

use regex::Regex;

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

struct TestCase {
    before: Vec<usize>,
    inst: Vec<usize>,
    after: Vec<usize>,
}

fn solve(input: &str, code: &str) -> (usize, usize) {
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

    let re = Regex::new(
        r"(?m)^Before: \[(\d+), (\d+), (\d+), (\d+)\]$
^(\d+) (\d+) (\d+) (\d+)$
^After:  \[(\d+), (\d+), (\d+), (\d+)\]$",
    )
    .unwrap();

    let test_cases = re
        .captures_iter(input)
        .map(|cap| {
            let vs = cap
                .iter()
                .skip(1)
                .map(|n| n.unwrap().as_str().parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            TestCase {
                before: vs[0..4].to_vec(),
                inst: vs[4..8].to_vec(),
                after: vs[8..12].to_vec(),
            }
        })
        .collect::<Vec<TestCase>>();

    let mut count = 0;
    let mut known: HashMap<usize, String> = HashMap::new();

    /* Build op code table */
    for case in &test_cases {
        let mut candidates = isa.iter().filter_map(|(n, f)| {
            let mut regs = case.before.clone();
            if f(&mut regs, case.inst[1], case.inst[2], case.inst[3]).is_ok() && regs == case.after {
                Some(n.to_owned())
            } else {
                None
            }
        }).collect::<HashSet<String>>();

        if candidates.len() >= 3 {
            count += 1;
        }

        for k in known.values() {
            candidates.remove(k);
        }

        if candidates.len() == 1 {
            println!("OP code {} is {:?}", case.inst[0], candidates);
            known.insert(case.inst[0], candidates.iter().next().unwrap().to_string());
        }
    }

    /* Run code */
    let mut registers = vec![0;4];
    for l in code.lines() {
        let vs = l.split_whitespace().map(|t| t.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let exec = isa.get(known.get(&vs[0]).unwrap()).unwrap();
        assert!(exec(&mut registers, vs[1], vs[2], vs[3]).is_ok());
    }

    (count, registers[0])
}

fn main() {
    let samples = include_str!("input-samples.txt");
    let code = include_str!("input-code.txt");
    println!("{:?}", solve(samples, code));
}
