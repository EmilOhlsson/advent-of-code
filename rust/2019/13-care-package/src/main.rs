pub mod intmachine;

#[derive(PartialEq, Eq)]
enum GameObject {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl From<i64> for GameObject {
    fn from(c: i64) -> Self {
        use GameObject::*;
        match c {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => Paddle,
            4 => Ball,
            _ => panic!(),
        }
    }
}

fn sign_of(x: i64) -> i64 {
    (x > 0) as i64 - (x < 0) as i64
}

fn solve(input: &str) -> usize {
    let mut screen = std::collections::HashMap::<(i64, i64), GameObject>::new();
    let mut machine = intmachine::Intmachine::load(input);
    let output = machine.run(&[]);
    assert!(output.len() % 3 == 0);

    for i in (0..output.len()).step_by(3) {
        screen.insert((output[i], output[i + 1]), GameObject::from(output[i + 2]));
    }

    screen.values().filter(|&o| *o == GameObject::Block).count()
}

fn solve_v2(input: &str) -> i64 {
    let mut machine = intmachine::Intmachine::load(input);

    machine.set_addr(0, 2); // Insert two coins, to play for free

    let mut buf = [0, 0, 0];
    let mut buf_i = 0;
    let mut score = 0;

    let mut pad_x = 0;
    let mut ball_x = 0;

    loop {
        use intmachine::IOState::*;
        use GameObject::*;

        // Always move pad towards ball
        match machine.run_to_event(Some(sign_of(ball_x - pad_x))) {
            Done => 
                return score,
            Input => (),
            Output(v) => {
                // Push value to buffer
                buf[buf_i] = v;
                buf_i += 1;
                if buf_i >= 3 {
                    // When buffer is full, parse
                    if buf[0] == -1 && buf[1] == 0 {
                        score = buf[2];
                    } else {
                        let object = GameObject::from(buf[2]);
                        match object {
                            Paddle => pad_x = buf[0],
                            Ball => ball_x = buf[0],
                            _ => (),
                        }
                    }

                    buf_i = 0;
                }
            }
        }
    }
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input));
    println!("{}", solve_v2(input));
}
