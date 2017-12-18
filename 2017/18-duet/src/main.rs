use std::collections::HashMap;
use std::collections::LinkedList;

fn get_value(regs: &mut HashMap<String, isize>, r: &String) -> isize {
    match r.parse::<isize>() {
        Ok(v) => v,
        Err(_) => {
            let v = regs.entry(r.clone()).or_insert(0);
            *v
        }
    }
}

fn run_program(input: &str) -> isize {
    let mut regs = [
        HashMap::<String, isize>::new(),
        HashMap::<String, isize>::new(),
    ];
    let instructions = input.lines().collect::<Vec<&str>>();
    let mut pc = [0, 0];
    let mut queue = [LinkedList::new(), LinkedList::new()];
    let mut done = [false, false];

    regs[0].insert(String::from("p"), 0);
    regs[1].insert(String::from("p"), 1);
    let mut sent_by_1 = 0;

    loop {
        let mut inc = [1, 1];
        for id in 0..pc.len() {
            if let Some(line) = instructions.get(pc[id] as usize) {
                let toks = line.trim()
                    .split_whitespace()
                    .map(String::from)
                    .collect::<Vec<String>>();
                match toks[0].as_str() {
                    "snd" => {
                        let v = get_value(&mut regs[id], &toks[1]);
                        // println!("[{}] sending {}", id, v);
                        queue[if id == 0 { 1 } else { 0 }].push_front(v);
                        if id == 1 {
                            sent_by_1 += 1;
                        }
                    }
                    "set" => {
                        let v = get_value(&mut regs[id], &toks[2]);
                        regs[id].insert(toks[1].clone(), v);
                    }
                    "add" => {
                        let y = get_value(&mut regs[id], &toks[2]);
                        let x = regs[id].entry(toks[1].clone()).or_insert(0);
                        *x += y;
                    }
                    "mul" => {
                        let y = get_value(&mut regs[id], &toks[2]);
                        let x = regs[id].entry(toks[1].clone()).or_insert(0);
                        *x *= y;
                    }
                    "mod" => {
                        let y = get_value(&mut regs[id], &toks[2]);
                        let x = regs[id].entry(toks[1].clone()).or_insert(0);
                        *x %= y;
                    }
                    "rcv" => {
                        if let Some(v) = queue[id].pop_back() {
                            // println!("[{}] Got {}", id, v);
                            regs[id].insert(toks[1].clone(), v);
                        } else {
                            // println!("[{}] Waiting", id);
                            // NOP until there is a value
                            inc[id] = 0;
                        }
                    }
                    "jgz" => {
                        let x = get_value(&mut regs[id], &toks[1]);
                        let y = get_value(&mut regs[id], &toks[2]);
                        if x > 0 {
                            inc[id] = y;
                        }
                    }
                    _ => panic!("Erhmagawd... {}", toks[0]),
                }
                pc[id] += inc[id];
            } else {
                done[id] = true;
            }
        }
        // If both programs done, or deadlock detected
        if (done[0] || done[1]) || (inc[0] == 0 && inc[1] == 0) {
            return sent_by_1;
        }
    }
}

fn main() {
    let program = include_str!("input.txt");
    println!("{}", run_program(program));
}

#[test]
fn test_program() {
    let program = include_str!("input-simple.txt");
    assert_eq!(run_program(program), 3);
}

#[test]
fn test_queue() {
    let mut queue = LinkedList::new();
    queue.push_front(1);
    queue.push_front(2);
    queue.push_front(3);
    queue.push_front(4);
    assert_eq!(queue.pop_back(), Some(1));
    assert_eq!(queue.pop_back(), Some(2));
    assert_eq!(queue.pop_back(), Some(3));
    assert_eq!(queue.pop_back(), Some(4));
    assert_eq!(queue.pop_back(), None);
}
