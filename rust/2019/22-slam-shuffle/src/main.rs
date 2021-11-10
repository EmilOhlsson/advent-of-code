use num::bigint::BigInt;
use num::bigint::ToBigInt;
use num::traits::{One, Zero};
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

fn polypow(a: BigInt, b: BigInt, m: BigInt, n: BigInt) -> (BigInt, BigInt) {
    if m == Zero::zero() {
        (One::one(), Zero::zero())
    } else if m.clone() % 2 == Zero::zero() {
        polypow(
            a.clone() * a.clone() % n.clone(),
            (a.clone() * b.clone() + b.clone()) % n.clone(),
            m.clone() / 2,
            n.clone(),
        )
    } else {
        let (c, d) = polypow(a.clone(), b.clone(), m.clone() - 1, n.clone());
        (a.clone() * c % n.clone(), (a.clone() * d + b) % n)
    }
}

fn solve_v2(input: &str, size: BigInt, times: BigInt, card: BigInt) -> BigInt {
    let cut_re = Regex::new(r"cut (-?\d+)").unwrap();
    let deal_re = Regex::new(r"deal into new stack").unwrap();
    let incr_re = Regex::new(r"deal with increment (-?\d+)").unwrap();

    // store as y = k * x + m as k and m
    let mut k: BigInt = One::one();
    let mut m: BigInt = Zero::zero();

    // Parse the lines in revers, and translate to inverse
    // of rule
    for line in input.lines().rev() {
        if let Some(cut) = cut_re.captures(line) {
            let v = cut[1].parse::<BigInt>().unwrap();
            m = (m + v) % size.clone();
        } else if let Some(incr) = incr_re.captures(line) {
            let v = incr[1].parse::<BigInt>().unwrap();
            let vp = v.modpow(&(size.clone() - 2.to_bigint().unwrap()), &size);
            k = k * vp.clone() % size.clone();
            m = m * vp.clone() % size.clone();
        } else if deal_re.captures(line).is_some() {
            k = -k;
            m = size.clone() - m - 1
        } else {
            panic!("line doesn't match: {:?}", line);
        }
    }

    let (kp, mp) = polypow(k, m, times, size.clone());
    (card * kp + mp) % size
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input, 10_007));
    println!(
        "{}",
        solve_v2(
            input,
            119_315_717_514_047i64.to_bigint().unwrap(),
            101_741_582_076_661i64.to_bigint().unwrap(),
            2_020i128.to_bigint().unwrap()
        )
    );
}
