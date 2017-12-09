#[derive(Clone,PartialEq,Eq)]
enum Kind {
    Group,
    Garbage,
}

fn score_stream(stream: &str) -> (usize, usize) {
    let mut state = Kind::Group;
    let mut score = 0;
    let mut depth = 0;
    let mut removed = 0;
    let mut escaped = false;

    for ch in stream.chars() {
        if state == Kind::Garbage && escaped {
            escaped = false;
            continue;
        }
        state = match state {
            Kind::Group => {
                match ch {
                    '{' => {
                        depth += 1;
                        Kind::Group
                    },
                    '}' => {
                        score += depth;
                        depth -= 1;
                        Kind::Group
                    },
                    '<' => Kind::Garbage,
                    ',' => Kind::Group,
                    _ => panic!("Unexpected {}", ch),
                }
            },
            Kind::Garbage => {
                match ch {
                '>' => Kind::Group,
                '!' => {escaped = true; Kind::Garbage},
                _ => {removed += 1; Kind::Garbage},
                }
            },
        }
    }
    (score, removed)
}

fn main() {
    let (input_score, removed) = score_stream(include_str!("input.txt").trim());
    println!("{}, {}", input_score, removed);
}

#[test]
fn test_inputs() {
    assert_eq!(score_stream("{}"), 1);
    assert_eq!(score_stream("{{{}}}"), 6);
    assert_eq!(score_stream("{{},{}}"), 5);
    assert_eq!(score_stream("{{{},{},{{}}}}"), 16);
    assert_eq!(score_stream("{<a>,<a>,<a>,<a>}"), 1);
    assert_eq!(score_stream("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
    assert_eq!(score_stream("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
    assert_eq!(score_stream("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
}
