use std::collections::VecDeque;
use std::rc::Rc;
use std::cmp;

fn search(length: usize, strength: usize, endpoint: usize, available: &VecDeque<Rc<(usize, usize)>>) -> (usize, usize) {
    let mut candidates = VecDeque::new();
    let mut left: VecDeque<Rc<(usize, usize)>> = VecDeque::new();

    for p in available {
        if p.0 == endpoint || p.1 == endpoint {
            candidates.push_back(Rc::clone(p));
        } else {
            left.push_back(Rc::clone(p));
        }
    }

    let mut best = (length, strength);
    for c in &candidates {
        let ep =
            if c.0 == endpoint { c.1 } else { c.0 };
        let available_tmp: VecDeque<Rc<(usize, usize)>> = left.iter()
            .chain(candidates.iter().filter(|&p| *p != *c))
            .cloned()
            .collect::<VecDeque<Rc<(usize, usize)>>>();
        best = cmp::max(best, search(length + 1, strength + c.0 + c.1, ep, &available_tmp));
    }

    best
}

fn find_longest(input: &str) -> usize {
    let available = input
        .lines()
        .map(|l| {
            let ts = l.split('/')
                .map(|t| t.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            Rc::new((ts[0], ts[1]))
        })
        .collect::<VecDeque<Rc<(usize, usize)>>>();

    search(0,0, 0, &available).1
}

fn main() {
    println!("{}", find_longest(include_str!("input")));
}

#[test]
fn test_code() {
    let input = include_str!("input-simple");
    assert_eq!(find_longest(input), 19);
}
