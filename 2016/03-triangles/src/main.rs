fn valid_triangles(a: usize, b: usize, c: usize) -> bool {
    (a + b > c) && (c + b > a) && (a + c > b)
}

#[test]
fn impossible() {
    assert_eq!(valid_triangles(5, 10, 25), false);
}

#[test]
fn simple() {
    assert_eq!(valid_triangles(1, 1, 1), true);
}

fn main() {
    let input = include_str!("../input.txt");

    let mut count: usize = 0;
    let triangles: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|t| t.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        }).collect();
    for i in 0..(triangles.len() / 3) {
        for j in 0..3 {
            count += if valid_triangles(
                triangles[i * 3][j],
                triangles[i * 3 + 1][j],
                triangles[i * 3 + 2][j],
            ) {
                1
            } else {
                0
            }
        }
    }
    println!("{}", count);
}
