use std::collections::HashMap;

enum State {
    A, B, C, D, E, F,
}

#[derive(PartialEq, Eq)]
enum Value {
    Zero, One,
}

impl State {
    fn read(&self, v: &mut Value) -> (State, isize) {
        match *self {
            State::A => {
                if *v == Value::Zero {
                    *v = Value::One;
                    (State::B, 1)
                } else {
                    *v = Value::Zero;
                    (State::B, -1)
                }
            }
            State::B => {
                if *v == Value::Zero {
                    (State::C, 1)
                } else {
                    (State::B, -1)
                }
            }
            State::C => {
                if *v == Value::Zero {
                    *v = Value::One;
                    (State::D, 1)
                } else {
                    *v = Value::Zero;
                    (State::A, -1)
                }
            }
            State::D => {
                if *v == Value::Zero {
                    *v = Value::One;
                    (State::E, -1)
                } else {
                    (State::F, -1)
                }
            }
            State::E => {
                if *v == Value::Zero {
                    *v = Value::One;
                    (State::A, -1)
                } else {
                    *v = Value::Zero;
                    (State::D, -1)
                }
            }
            State::F => {
                if *v == Value::Zero {
                    *v = Value::One;
                    (State::A, 1)
                } else {
                    (State::E, -1)
                }
            }
        }
    }
}

fn main() {
    let steps = 12_629_077;
    let mut state = State::A;
    let mut ptr = 0;
    let mut tape: HashMap<isize, Value> = HashMap::new();
    for _ in 0..steps {
        let v = tape.entry(ptr).or_insert(Value::Zero);
        let (new_state, inc) = state.read(v);

        state = new_state;
        ptr += inc;
    }
    println!("{}", tape.values().map(|v| if v == &Value::Zero { 0 } else { 1}).sum::<usize>());
}
