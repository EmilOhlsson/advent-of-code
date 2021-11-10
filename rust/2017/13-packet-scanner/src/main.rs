fn parse_input(desc: &str) -> Vec<(isize, isize)> {
    desc.lines().map(|l| {
        let ts: Vec<isize> = l.split(':').map(|t| t.trim().parse::<isize>().unwrap()).collect();
        (ts[0], ts[1])
    }).collect()
}

fn severity(firewall: &Vec<(isize, isize)>) -> isize {
    firewall.iter().filter(|&&(position, height)| {
        position % (2 * (height - 1)) == 0
    }).map(|&(position, height)| position * height).sum()
}

fn safe_delay(firewall: &Vec<(isize, isize)>) -> isize {
    (0..)
        .find(|w| !firewall.iter().any(|&(p, h)| (p + w) % (2 * (h - 1)) == 0))
        .unwrap()
}

fn main() {
    let input = parse_input(include_str!("input.txt").trim());
    println!("{}", severity(&input));
    println!("{}", safe_delay(&input));
}

#[test]
fn test_simple() {
    let input = parse_input(include_str!("input-simple.txt").trim());
    assert_eq!(severity(&input), 24);
    assert_eq!(safe_delay(&input), 10);
}