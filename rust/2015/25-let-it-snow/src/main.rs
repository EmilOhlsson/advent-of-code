fn next(v: u64) -> u64 {
    (v * 252533) % 33554393
}

fn get_pos(row: u64, col: u64) -> u64 {
    let n = row + col - 2;
    n * (n + 1) / 2 + col
}

fn solve(row: u64, col: u64) -> u64 {
    let mut code = 20151125;
    for _ in 1..get_pos(row, col) {
        code = next(code);
    }
    code
}

fn main() {
    println!("{}", solve(1, 6));
    println!("{}", solve(2947, 3029));
}
