fn find_loop(public: u64) -> u64 {
    let mut v = 1;
    for l in 1.. {
        v = (v * 7) % 20201227;
        if v == public {
            return l;
        }
    }
    panic!("Did not find loop value");
}

fn run_loop(subj: u64, loops: u64) -> u64 {
    let mut v = 1;
    for _ in 0..loops {
        v = (v * subj) % 20201227;
    }

    v
}

fn solve(door: u64, key: u64) -> u64 {
    let key_loops = find_loop(key);
    run_loop(door, key_loops)
}

fn main() {
    println!("{}", solve(5099500, 7648211));
}
