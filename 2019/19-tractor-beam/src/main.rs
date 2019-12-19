pub mod intmachine;
use intmachine::Intmachine;

fn solve(input: &str) -> u32 {
    let machine = Intmachine::load(input, 10);
    let mut sum = 0;
    for y in 0..50 {
        for x in 0..50 {
            if machine.clone().run(&[x, y])[0] > 0 {
                print!("#");
                sum += 1;
            } else {
                print!(".");
            }
        }
        println!();
    }
    sum
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input));
}
