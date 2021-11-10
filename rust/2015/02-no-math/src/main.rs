fn tulip(l: Vec<usize>) -> (usize, usize, usize) {
    (l[0], l[1], l[2])
}

fn solve(input: &str) -> (usize, usize) {
    let mut area = 0usize;
    let mut ribbon = 0usize;
    for line in input.lines() {
        let (l, w, h) = tulip(line.split('x').map(|t| t.parse::<usize>().unwrap()).collect::<Vec<usize>>());

        let sides = vec![l * w, w * h, h * l];
        let perims = vec![l + w, w + h, h + l];

        ribbon += w * l * h + 2 * perims.iter().min().unwrap();
        let min = sides.iter().min().unwrap();
        area += sides.iter().map(|&s| 2 * s).sum::<usize>() + min;
    }
    return (area, ribbon);
}

fn main() {
    let input = include_str!("input");
    let (area, ribbon) = solve(&input);

    println!("{}", area);
    println!("{}", ribbon);
}

#[test]
fn test_input() {
    assert_eq!(solve("2x3x4"), (58, 34));
    assert_eq!(solve("1x1x10"), (43, 14));
}
