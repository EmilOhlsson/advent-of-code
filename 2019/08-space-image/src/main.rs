fn idx(l: u32, r: u32, c: u32, width: u32, height: u32) -> usize {
    ((l * width * height) + r * width + c) as usize
}

fn solve_p1(input: &str, width: u32, height: u32) -> usize {
    let inp = input
        .to_string()
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let nlayers = inp.len() as u32 / width / height;
    let mut layers: Vec<Vec<Vec<u32>>> = Vec::new();
    for l in 0..nlayers {
        let mut lines: Vec<Vec<u32>> = Vec::new();
        for r in 0..height {
            let line = inp[idx(l, r, 0, width, height)..idx(l, r + 1, 0, width, height)].to_vec();
            lines.push(line);
        }

        layers.push(lines);
    }

    let layer = layers
        .iter()
        .min_by_key(|l| l.iter().flatten().filter(|&d| *d == 0).count())
        .unwrap();

    let ones = layer.iter().flatten().filter(|&d| *d == 1).count();
    let twos = layer.iter().flatten().filter(|&d| *d == 2).count();

    for r in 0..height {
        for c in 0..width {
            let mut pixel = 0;
            for l in 0..nlayers {
                pixel = match layers[(nlayers - 1 -l) as usize][r as usize][c as usize] {
                    0 => 0,
                    1 => 1,
                    2 => pixel,
                    _ => panic!("Invalid pixel in layer"),
                }
            }
            print!("{}", match pixel {
                0 => ' ',
                1 => '*',
                _ => panic!(),
            });
        }
        println!();
    }

    ones * twos
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve_p1(input, 25, 6));

}

#[test]
fn test() {
    assert_eq!(solve_p1("123456789012", 3, 2), 1);
}
