use std::collections::LinkedList;
use std::rc::Rc;
use std::cmp;

fn search(strength: usize, endpoint: usize, available: LinkedList<Rc<(usize, usize)>>) -> usize {
    let mut candidates = LinkedList::new();
    let mut left: LinkedList<Rc<(usize, usize)>> = LinkedList::new();

    for p in &available {
        if p.0 == endpoint || p.1 == endpoint {
            candidates.push_back(p.clone());
        } else {
            left.push_back(p.clone());
        }
    }

    let mut strength_best = strength;
    for c in &candidates {
        let ep =
            if c.0 == endpoint { c.1 } else { c.0 };
        let available_tmp: LinkedList<Rc<(usize, usize)>> = left.iter()
            .chain(candidates.iter().filter(|&p| *p != *c))
            .cloned()
            .collect::<LinkedList<Rc<(usize, usize)>>>();
        let strength_tmp = search(strength + c.0 + c.1, ep, available_tmp);
        strength_best = cmp::max(strength_best, strength_tmp);
    }

    strength_best
}

fn find_strongest(input: &str) -> usize {
    let available = input
        .lines()
        .map(|l| {
            let ts = l.split('/')
                .map(|t| t.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            Rc::new((ts[0], ts[1]))
        })
        .collect::<LinkedList<Rc<(usize, usize)>>>();

    search(0, 0, available)
}

fn main() {
    println!("{}", find_strongest(include_str!("input")));
}

#[test]
fn test_code() {
    let input = include_str!("input-simple");
    assert_eq!(find_strongest(input), 31);
}
