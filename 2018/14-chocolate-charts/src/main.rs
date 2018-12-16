fn sum_to_array(mut sum: usize) -> Vec<usize> {
    let mut v = Vec::new();
    if sum == 0 {
        vec![0]
    } else {
        while sum > 0 {
            v.push(sum % 10);
            sum /= 10;
        }
        v.reverse();
        v
    }
}

fn solve_p1(input: usize) -> String {
    let mut scores = vec![3usize, 7usize];
    let mut gnomes = (0, 1);

    while scores.len() < input + 10 {
        let mut v = sum_to_array(scores[gnomes.0] + scores[gnomes.1]);
        scores.append(&mut v);

        // Move gnomes;
        let l = scores.len();
        gnomes = (
            (gnomes.0 + scores[gnomes.0] + 1) % l,
            (gnomes.1 + scores[gnomes.1] + 1) % l,
        );
    }

    scores
        .iter()
        .skip(input)
        .take(10)
        .map(|n| format!("{}", n))
        .collect()
}

fn solve_p2(input: &str) -> usize {
    let mut scores = vec![3usize, 7usize];
    let mut gnomes = (0, 1);
    let inp = input
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    loop {
        let mut v = sum_to_array(scores[gnomes.0] + scores[gnomes.1]);
        scores.append(&mut v);

        // Move gnomes;
        let l = scores.len();
        gnomes = (
            (gnomes.0 + scores[gnomes.0] + 1) % l,
            (gnomes.1 + scores[gnomes.1] + 1) % l,
        );

        if scores.len() > inp.len() + 1 {
            let l0 = scores.len();
            let l1 = inp.len();
            if scores[(l0 - l1)..l0] == inp[..] {
                return l0 - l1;
            } else if scores[(l0 - l1 - 1)..(l0 - 1)] == inp[..] {
                return l0 - l1 - 1;
            }
        }
    }
}

fn main() {
    let input = "640441";
    println!("{:?}", solve_p1(input.parse::<usize>().unwrap()));
    println!("{:?}", solve_p2(input));
}

#[test]
fn test() {
    assert_eq!(solve_p1(9), "5158916779");
    assert_eq!(solve_p1(5), "0124515891");
    assert_eq!(solve_p1(18), "9251071085");
    assert_eq!(solve_p1(2018), "5941429882");

    assert_eq!(solve_p2("51589"), 9);
    assert_eq!(solve_p2("01245"), 5);
    assert_eq!(solve_p2("92510"), 18);
    assert_eq!(solve_p2("59414"), 2018);
}
