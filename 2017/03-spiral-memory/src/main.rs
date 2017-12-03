const ADDRESS: usize = 277678;

fn spiral(addr: usize) -> (isize, isize) {
    let mut p = (0, 0);
    let mut dp = (0, -1);

    for _ in 1..addr {
        if p.0 == p.1 || (p.0 < 0 && p.0 == -p.1) || (p.0 > 0 && p.0 == 1 - p.1) {
            dp = (-dp.1, dp.0);
        }
        p = (p.0 + dp.0, p.1 + dp.1);
    }
    return p;
}

fn manhattan_distance(p: (isize, isize)) -> usize {
    (p.0.abs() + p.1.abs()) as usize
}

fn main() {
    println!("{}", manhattan_distance(spiral(ADDRESS)));
}
