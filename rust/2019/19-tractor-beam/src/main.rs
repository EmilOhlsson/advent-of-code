pub mod intmachine;
use intmachine::Intmachine;

fn solve(input: &str) -> u32 {
    let machine = Intmachine::load(input, 10);
    let mut sum = 0;
    for y in 0..50 {
        for x in 0..50 {
            if machine.clone().run(&[x, y])[0] > 0 {
                sum += 1;
            }
        }
    }
    sum
}

fn solve_v2(input: &str) -> i64 {
    let machine = Intmachine::load(input, 0);
    for y in 1_000.. {
        for x in 3 * y / 5.. {
            if machine.clone().run(&[x, y])[0] > 0 {
                if machine.clone().run(&[x + 99, y - 99])[0] > 0 {
                    return x * 10_000 + y - 99;
                }
                break;
            }
        }
    }

    panic!()
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input));
    println!("{}", solve_v2(input));
}
