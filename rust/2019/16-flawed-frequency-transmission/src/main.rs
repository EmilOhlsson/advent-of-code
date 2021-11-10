#[derive(Debug)]
struct VecInt {
    digits: Vec<i32>,
}

impl VecInt {
    fn from_str(num: &str) -> VecInt {
        let digits = num
            .trim()
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>();
        VecInt { digits }
    }

    fn filter(&self) -> VecInt {
        let mut next = Vec::new();
        for i in 1..=self.digits.len() {
            let sum = self
                .digits
                .iter()
                .zip(
                    [0, 1, 0, -1]
                        .iter()
                        .map(|v| std::iter::repeat(*v).take(i))
                        .flatten()
                        .cycle()
                        .skip(1),
                )
                .map(|(d, f)| (d * f) as i128)
                .sum::<i128>()
                .abs()
                % 10;
            next.push(sum as i32);
        }
        VecInt { digits: next }
    }
}

fn solve(input: &str, phases: u32) -> String {
    let mut digits = VecInt::from_str(input);

    for _ in 0..phases {
        let next = digits.filter();
        digits = next;
    }
    digits
        .digits
        .iter()
        .take(8)
        .map(|d| std::char::from_digit(*d as u32, 10).unwrap())
        .collect::<String>()
}

fn phase(input: Vec<i32>) -> Vec<i32> {
    let mut output = input.clone();
    let mut sum = 0;
    for i in (0..input.len()).rev() {
        sum = (sum + input[i]) % 10;
        output[i] = sum;
    }
    output
}

fn fft(mut digits: Vec<i32>, phases: u32) -> Vec<i32> {
    for _ in 0..phases {
        digits = phase(digits);
    }
    digits
}

fn solve_v2(input: &str, phases: u32) -> String {
    let digits =
        VecInt::from_str(&std::iter::repeat(input).take(10_000).collect::<String>()).digits;
    let offset = input[0..7].parse::<usize>().unwrap();
    let repeats = 10_000;

    let input_len = input.len();
    let input: Vec<i32> = std::iter::repeat(digits.clone())
        .flatten()
        .skip(offset % input_len)
        .take(input_len * repeats - offset)
        .collect();
    fft(input, phases)[0..8]
        .iter()
        .map(|d| std::char::from_digit(*d as u32, 10).unwrap())
        .collect()
}

fn main() {
    let input = include_str!("input").trim();
    println!("{}", solve(input, 100));
    println!("{}", solve_v2(&input, 100));
}

#[test]
fn test_vec_int() {
    let digits = VecInt::from_str("123");
    assert_eq!(digits.digits, [1, 2, 3]);
}

#[test]
fn test_simple() {
    assert_eq!(solve("12345678", 4), "01029498");
}

#[test]
fn test0() {
    let input = include_str!("input-test0");
    assert_eq!(solve(input, 100), "24176176");
}
#[test]
fn test1() {
    let input = include_str!("input-test1");
    assert_eq!(solve(input, 100), "73745418");
}
#[test]
fn test2() {
    let input = include_str!("input-test2");
    assert_eq!(solve(input, 100), "52432133");
}

#[test]
fn test0_v2() {
    assert_eq!(
        solve_v2("03036732577212944063491565474664", 100),
        "84462026"
    );
}

#[test]
fn test1_v2() {
    assert_eq!(
        solve_v2("02935109699940807407585447034323", 100),
        "78725270"
    );
}

#[test]
fn test2_v2() {
    assert_eq!(
        solve_v2("03081770884921959731165446850517", 100),
        "53553731"
    );
}
