fn solve(n: usize) -> usize {
    // Primes will get the least amount of prestents (10 * (p + 1)). So pretend that 4 million is a
    // prime (there will be a prime between n and 4 million
    let mut presents = vec![0usize; 4_000_000];
    for i in 1.. {
        presents.iter_mut().skip(i).step_by(i).for_each(|p| *p += i);
        if presents[i] * 10 >= n {
            return i;
        }
    }
    panic!("broken :(");
}

fn solve_v2(n: usize) -> usize {
    let mut presents = vec![0usize; 4_000_000];
    for i in 1.. {
        presents
            .iter_mut()
            .skip(i)
            .step_by(i)
            .take(50)
            .for_each(|p| *p += i);
        if presents[i] * 11 >= n {
            return i;
        }
    }
    panic!("broken :(");
}

fn main() {
    println!("{}", solve(34000000));
    println!("{}", solve_v2(34000000));
}
