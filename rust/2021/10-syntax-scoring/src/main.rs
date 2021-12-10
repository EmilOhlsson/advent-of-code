enum Line {
    Corrupt(u64),
    Incomplete(u64),
}

fn get_matching(ch: char) -> char {
    match ch {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("No match for {:?}", ch),
    }
}

fn get_corrupt_score(ch: char) -> Line {
    match ch {
        ')' => Line::Corrupt(3),
        ']' => Line::Corrupt(57),
        '}' => Line::Corrupt(1197),
        '>' => Line::Corrupt(25137),
        _ => panic!("No score for {:?}", ch),
    }
}

fn get_incomplete_score(ch: char) -> u64 {
    match ch {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("No score for {:?}", ch),
    }
}

impl Line {
    fn as_corrupt(self) -> Option<u64> {
        if let Line::Corrupt(v) = self {
            Some(v)
        } else {
            None
        }
    }
    fn as_incomplete(self) -> Option<u64> {
        if let Line::Incomplete(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

fn score_line(line: &str) -> Line {
    let mut stack = Vec::new();
    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ')' | ']' | '}' | '>' => {
                if let Some(opening) = stack.pop() {
                    if ch != get_matching(opening) {
                        return get_corrupt_score(ch);
                    }
                } else {
                    return get_corrupt_score(ch);
                }
            }
            _ => panic!("Did not expect the spanish inquisition")
        }
    }
    
    /* Calculate incomplete score */
    let mut score = 0;
    while let Some(top) = stack.pop() {
        score *= 5;
        score += get_incomplete_score(get_matching(top));
        
    }
    Line::Incomplete(score)
}

fn solve_p1(input: &str) -> u64 {
    input.lines().map(score_line).filter_map(Line::as_corrupt).sum()
}

fn solve_p2(input: &str) -> u64 {
    let mut scores = input.lines().map(score_line).filter_map(Line::as_incomplete).collect::<Vec<u64>>();
    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve_p1(input));
    println!("{}", solve_p2(input));
}

#[test]
fn test_simple() {
    let input = include_str!("input-simple");
    assert_eq!(solve_p1(input), 26397);
    assert_eq!(solve_p2(input), 288957);
}
