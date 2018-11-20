use std::collections::VecDeque;

fn solve_p1(n: usize) -> usize {
    let mut gnomes: VecDeque<usize> = VecDeque::new();
    for i in 1..=n {
        gnomes.push_back(i);
    }

    loop {
        let g1 = gnomes.pop_front().unwrap();
        let g2 = gnomes.pop_front();
        if g2.is_none() {
            return g1;
        }
        gnomes.push_back(g1);
    }
}

fn solve_p2(n: usize) -> usize {
    let mut gnomes: VecDeque<usize> = VecDeque::new();
    for i in 1..=n {
        gnomes.push_back(i);
    }

    loop {
        let len = gnomes.len();
        gnomes.remove(len / 2);

        if gnomes.len() == 1 {
            return gnomes[0];
        }
        let gnome = gnomes.pop_front().unwrap();
        gnomes.push_back(gnome);
    }
}

fn main() {
    let key = 3_005_290;
    println!("{}", solve_p1(key));
    println!("{}", solve_p2(key));
}

#[test]
fn test() {
    assert_eq!(solve_p1(5), 3);
    assert_eq!(solve_p2(5), 2);
}
