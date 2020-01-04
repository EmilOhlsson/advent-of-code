pub mod intmachine;

use intmachine::Intmachine;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Packet {
    x: i64,
    y: i64,
}

fn solve(input: &str) -> i64 {
    let mut nics = Vec::new();
    let mut queues = Vec::new();
    let mut nat = None;
    let mut nat_prev = None;
    let mut p1 = false;

    for i in 0..50 {
        let mut machine = Intmachine::load(input, 1024);
        machine.push_input(i);
        nics.push(machine);
        queues.push(VecDeque::<Packet>::new());
    }

    for _i in 0.. {
        for i in 0..50 {
            let nic = nics.get_mut(i).unwrap();
            match nic.run_to_event() {
                intmachine::IOState::Output(addr) => {
                    let packet = Packet {
                        x: nic.run_to_event().unwrap(),
                        y: nic.run_to_event().unwrap(),
                    };
                    if addr == 255 {
                        if !p1 {
                            println!("part1: {:?}", packet);
                            p1 = true;
                        }
                        nat = Some(packet);
                    } else {
                        queues[addr as usize].push_back(packet);
                    }
                }

                intmachine::IOState::Done => panic!("Unexpected program exit"),
                intmachine::IOState::Input => {
                    if let Some(packet) = queues[i].pop_front() {
                        nic.push_input(packet.x);
                        nic.push_input(packet.y);
                    } else {
                        nic.push_input(-1);
                    }
                }
            }
        }

        if nat.is_some() && queues.iter().map(|q| q.len()).sum::<usize>() == 0 {
            if nat_prev == nat {
                println!("[{}] {:?}", _i, nat);
            }
            nat_prev = nat;
            queues[0].push_back(nat.unwrap());
            nat = None;
        }
    }

    unreachable!()
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input));
}
