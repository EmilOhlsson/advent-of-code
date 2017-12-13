
fn parse_input(desc: &str) -> Vec<(isize, isize)> {
    desc.lines().map(|l| {
        let ts: Vec<isize> = l.split(':').map(|t| t.trim().parse::<isize>().unwrap()).collect();
        (ts[0], ts[1])
    }).collect()
}

fn simulate(firewall: Vec<(isize, isize)>) -> isize {
    let mut scanners: Vec<isize> = vec![0; firewall.len()];
    let mut scanner_step: Vec<isize> = vec![0; firewall.len()];
    let mut severity = 0;

    let levels = firewall.iter().map(|l| l.0).max().unwrap() + 1;

    for level in 0..levels {
        for i in 0..scanners.len() {
            // Step and change direction if at end of layer
            scanners[i] = scanners[i] + scanner_step[i];
            if scanners[i] == 0 { scanner_step[i] = 1;}
            if scanners[i] == firewall[i].1 - 1 { scanner_step[i] = -1;}
            
            // Check if caught
            if level == firewall[i].0 && scanners[i] == 0 {
                severity += firewall[i].0 * firewall[i].1;
            }
        }
    }
    return severity;
}

fn main() {
    let input = parse_input(include_str!("input.txt").trim());
    println!("{}", simulate(input));
}

#[test]
fn test_simple() {
    let input = parse_input(include_str!("input-simple.txt").trim());
    assert_eq!(simulate(input), 24);
}