use regex::Regex;

fn wrap(v: i32, r: i32) -> i32 {
    if v < 0 {
        v + r
    } else {
        v
    }
}

fn modulo(n: i32, d: i32) -> i32 {
    if n >= 0 {
        n % d
    } else {
        d - (-n) % d
    }
}

fn solve(input: &str, size: i32) -> i32 {
    let deal_re = Regex::new(r"deal into new stack").unwrap();
    let cut_re = Regex::new(r"cut (-?\d+)").unwrap();
    let incr_re = Regex::new(r"deal with increment (-?\d+)").unwrap();
    let mut lo = 1;
    let mut hi = 0;
    for line in input.lines() {
        let l;
        let h;
        if let Some(cut) = cut_re.captures(line) {
            let v = cut[1].parse::<i32>().unwrap();
            l = 1;
            h = size - wrap(v, size);
        } else if let Some(incr) = incr_re.captures(line) {
            l = incr[1].parse::<i32>().unwrap();
            h = 0;
        } else if deal_re.captures(line).is_some() {
            l = -1;
            h = size - 1;
        } else {
            panic!("line doesn't match: {:?}", line);
        }
        lo = modulo(l * lo, size);
        hi = modulo(l * hi + h, size);
    }
    modulo(lo * 2019 + hi, size)
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input, 10_007));
}
