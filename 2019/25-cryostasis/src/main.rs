pub mod intmachine;

use intmachine::{IOState, Intmachine};
use std::io;

fn main() {
    let mut robot = Intmachine::load(include_str!("input"), 1024);

    loop {
        match robot.run_to_event() {
            IOState::Done => return,
            IOState::Output(o) => print!("{}", o as u8 as char),
            IOState::Input => {
                let mut input = String::new();
                io::stdin().read_line(&mut input).ok();
                input.chars().for_each(|ch| robot.push_input(ch as i64));
            }
        }
    }
}
