pub mod intmachine;

use intmachine::Intmachine;

fn solve(input: &str, program: &str) -> i64 {
    let mut machine = Intmachine::load(input);
    let instructions = program
        .lines()
        .map(|l| l.split('#').next().unwrap().trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();
    println!("Running program");
    for inst in &instructions {
        println!("  {}", inst);
    }
    let machine_code = instructions
        .iter()
        .map(|l| {
            format!("{}\n", l)
                .chars()
                .map(|ch| ch as i64)
                .collect::<Vec<i64>>()
        })
        .flatten()
        .collect::<Vec<i64>>();
    assert!(instructions.len() <= 15);
    let output = machine.run(&machine_code);
    let result = *output.last().unwrap();
    if result <= std::u8::MAX as i64 {
        let output_rep = output.iter().map(|&v| v as u8 as char).collect::<String>();
        println!("{}", output_rep.trim());
        println!("01234567890");
    }
    result
}

fn main() {
    let input = include_str!("input");
    let program_v1 = include_str!("springscript-v1");
    let program_v2 = include_str!("springscript-v2");
    println!("Part 1: {}", solve(input, program_v1));
    println!("Part 2: {}", solve(input, program_v2));
}
