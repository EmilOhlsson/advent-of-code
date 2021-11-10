use std::collections::HashMap;

type Matrix = Vec<Vec<bool>>;

fn flip_y(m: &Matrix) -> Matrix {
    let mut v: Matrix = Vec::new();
    for l in m {
        v.push(l.iter().rev().cloned().collect::<Vec<bool>>());
    }
    v
}

fn flip_x(m: &Matrix) -> Matrix {
    let mut v: Matrix = Vec::new();
    for l in m.iter().rev() {
        v.push(l.iter().cloned().collect::<Vec<bool>>());
    }
   v
}

#[test]
fn test_flip() {
    let input = vec![
        vec![true, false],
        vec![false, false],
    ];
    assert_eq!(
        flip_x(&input), 
        vec![
            vec![false, false],
            vec![true, false]
        ]);
    assert_eq!(
        flip_y(&input),
        vec![
            vec![false, true],
            vec![false, false]
        ]);
}

fn rotated(m: &Matrix) -> Matrix {
    let mut v: Matrix = Vec::new();
    let len = m.len();
    for c in 0..len {
        let mut tmp = Vec::new();
        for r in 0..len {
            tmp.push(m[r][c]);
        }
        tmp.reverse();
        v.push(tmp);
    }
   v
}

#[test]
fn test_rotation() {
    let input = vec![
        vec![true, false, false],
        vec![false, false, false],
        vec![false, false, false],
    ];
    assert_eq!(
        rotated(&input),
        vec![
            vec![false, false, true],
            vec![false, false, false],
            vec![false, false, false],
        ]);
}

fn variants(m: &Matrix) -> Vec<Matrix> {
    let mut res: Vec<Matrix> = Vec::new();
    let mut tmp: Matrix = m.clone();
    for _ in 0..4 {
        res.push(tmp.clone());
        res.push(flip_x(&tmp));
        res.push(flip_y(&tmp));
        tmp = rotated(&tmp);
    }
    res
}

fn submatrices(m: &Matrix) -> (usize, usize, Vec<Matrix>) {
    let mut result: Vec<Matrix> = Vec::new();
    let mut w_i = 2;
    while m.len() % w_i != 0 { w_i += 1}
    print_matrix(m, "I haz");
    let w_o = m.len() / w_i;
    println!("w_i: {}, w_o: {}", w_i, w_o);
    for _ in 0..(w_o * w_o) { result.push(Vec::new()); }

    // split rows in into chunks
    for (r, rows) in m.chunks(w_i).enumerate() {
        // For each row in chunk
        for row in rows {
            // Split row into chunks
            for (c, cols) in row.chunks(w_i).enumerate() {
                result[r * w_o + c].push(Vec::from(cols));
            }
        }
    }
    (w_i, w_o, result)
}

#[test]
fn test_submatrices_one() {
    let input = vec![
        vec![true, false],
        vec![false, false]
    ];
    let (w_i, w_o, one) = submatrices(&input);
    assert_eq!(one,
        vec![
            vec![
                vec![true, false],
                vec![false, false]
            ]
        ]);
    assert_eq!(w_i, 2);
    assert_eq!(w_o, 1);
}

#[test]
fn test_submatrices() {
    let input = vec![
        vec![true, false, true, false],
        vec![false, false, false, false], 
        vec![true, false, true, false],
        vec![false, false, false, false], 
    ];
    let (w_i, w_o, result) = submatrices(&input);
    assert_eq!(result,
        vec![
            vec![
                vec![true, false],
                vec![false, false]
            ],
            vec![
                vec![true, false],
                vec![false, false]
            ],
            vec![
                vec![true, false],
                vec![false, false]
            ],
            vec![
                vec![true, false],
                vec![false, false]
            ],
        ]);
    assert_eq!(w_i, 2);
    assert_eq!(w_o, 2);
}

#[test]
fn test_submatrices_initial() {
    let input: Matrix = 
        vec![
            vec![false, true, false],
            vec![false, false, true],
            vec![true, true, true]
        ];
    let (w_i, w_o, result) = submatrices(&input);
    assert_eq!(w_i, 3);
    assert_eq!(w_o, 1);
    assert_eq!(result[0], input);

}

fn merge(mm: &Vec<Matrix>, w_o: usize) -> Matrix {
    let mut result: Matrix = Vec::new();
    let w_i = mm[0].len();
    for rows in mm.chunks(w_o) {
        for r_i in 0..w_i {
            let mut row: Vec<bool> = Vec::new();
            for m in rows {
                let mut tmp = m[r_i].clone();
                row.append(&mut tmp);
            }
            result.push(row);
        }
    }
    result
}

#[test]
fn test_merge_one() {
    let input: Vec<Matrix> =
        vec![
            vec![
                vec![true, false, false, true],
                vec![false, false, false, false],
                vec![false, false, false, false],
                vec![true, false, false, true],
            ]
        ];
    let output = merge(&input, 1);
    assert_eq!(output, input[0]);
}

#[test]
fn test_merge() {
    let input: Vec<Matrix> =
        vec![
            vec![
                vec![true, false],
                vec![false, false]
            ],
            vec![
                vec![true, false],
                vec![false, false]
            ],
            vec![
                vec![true, false],
                vec![false, false]
            ],
            vec![
                vec![true, false],
                vec![false, false]
            ],
        ];
    let mtx = merge(&input, 2);
    assert_eq!(mtx, 
        vec![
            vec![true,  false, true,  false],
            vec![false, false, false, false], 
            vec![true,  false, true,  false],
            vec![false, false, false, false], 
        ]);

}

fn print_matrix(m: &Matrix, msg: &str) {
    println!("\n{}:", msg);
    for r in m {
        r.iter().for_each(|&p| print!("{}", if p { '#' } else { '.' }));
        println!();
    }
}

fn solve(input: &str, iterations: usize) -> usize {
    let mut translations: HashMap<Matrix, Matrix> = HashMap::new();
    for line in input.lines() {
        let segs = line.split(" => ")
            .map(|t| {
                t.split('/')
                    .map(|l| l.chars().map(|c| c == '#').collect::<Vec<bool>>())
                    .collect::<Matrix>()
            })
            .collect::<Vec<Matrix>>();
        print_matrix(&segs[0], "this");
        print_matrix(&segs[1], "maps to");
        let vs = variants(&segs[0]);
        vs.into_iter().for_each(|m| {
            translations.insert(m, segs[1].clone());
        });
    }

    let mut mtx: Matrix = vec![vec![false, true, false], vec![false, false, true], vec![true, true, true]];
    for _ in 0..iterations {
        print_matrix(&mtx, "Tick");
        println!("Finding submatrices");
        let (w_i, w_o, sm) = submatrices(&mtx);
        let new = sm.into_iter()
            .map(|m| translations[&m].clone())
            .inspect(|m| print_matrix(m, "got"))
            .collect::<Vec<Matrix>>();
        println!("w_i: {}, w_o: {}", w_i, w_o);
        for m in &new {
            print_matrix(&m, "translation result");
        }
        mtx = merge(&new, w_o);
    }

    mtx.iter().flat_map(|s| s.iter()).map(|&p| if p { 1 } else { 0 }).sum()
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input, 18));
}

#[test]
fn test_simple() {
    let input = include_str!("input-simple");
    assert_eq!(solve(input, 2), 12);
}
