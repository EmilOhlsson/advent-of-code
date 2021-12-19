use rayon::prelude::*;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

type Xyz = Vec<i32>;
type XyzSet = HashSet<[i32; 3]>;
type XyzVec = Vec<Xyz>;

/// Read input into a Vector of point sets
fn parse(input: &str) -> Vec<XyzVec> {
    input
        .split("\n\n")
        .map(|section| {
            section
                .lines()
                .filter(|line| !line.starts_with("---"))
                .map(|line| {
                    line.split(',')
                        .map(str::parse::<i32>)
                        .map(Result::unwrap)
                        .collect::<Xyz>()
                })
                .collect::<XyzVec>()
        })
        .collect()
}

// The transform Ax + b = y can be written as A'x = y, using
// [ A   | b ] [ x ]   [ y ]
// [     | . ] [ . ] = [ . ]
// [ 0.. | 1 ] [ 1 ]   [ 1 ]
//
// Inverse of a ortgonal matrix is it's transpose, and that
// seem to be that case for swizzle matrices.
//
// So the inverse of the matrix above is
//
// [ tr(A)  | -A^T * b ]
// [        |  ...     ]
// [ 0..    |     1    ]
type Transform = [[i32; 4]; 4];

/// Rotate transforms over x, y or z axis
fn rot_x(a: &Transform) -> Transform {
    mulm(
        &[[1, 0, 0, 0], [0, 0, -1, 0], [0, 1, 0, 0], [0, 0, 0, 1]],
        a,
    )
}

fn rot_y(a: &Transform) -> Transform {
    mulm(
        &[[0, 0, 1, 0], [0, 1, 0, 0], [-1, 0, 0, 0], [0, 0, 0, 1]],
        a,
    )
}

fn rot_z(a: &Transform) -> Transform {
    mulm(
        &[[0, -1, 0, 0], [1, 0, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]],
        a,
    )
}

static UNIT: Transform = [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]];

/// Compare two sets of coordinates. If there are at least 12 identical offsets,
/// then return that offset
fn compare_sets(s1: &XyzSet, s2: &XyzSet) -> Option<[i32; 3]> {
    let mut distances = HashMap::<[i32; 3], usize>::new();
    for a in s1 {
        for b in s2 {
            let dist = [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
            let entry = distances.entry(dist).or_insert(0);
            *entry += 1;
        }
    }

    let threshold = min(12, s1.len());
    for (o, c) in &distances {
        if *c >= threshold {
            return Some(*o);
        }
    }
    None
}

fn check_rotations(a: &[Xyz], b: &[Xyz]) -> Option<Transform> {
    // Compare the different dimensions
    let mut rot = UNIT;
    let set = transform(a, &UNIT);
    for _x in 0..4 {
        for _y in 0..4 {
            for _z in 0..4 {
                let ps = transform(b, &rot);
                if let Some(offset) = compare_sets(&set, &ps) {
                    rot[0][3] = offset[0];
                    rot[1][3] = offset[1];
                    rot[2][3] = offset[2];
                    return Some(rot);
                }
                rot = rot_z(&rot);
            }
            rot = rot_y(&rot);
        }
        rot = rot_x(&rot);
    }
    assert_eq!(rot, UNIT);
    None
}

/// Tranform one array of points to a XyzSet using transform
fn transform(vec: &[Xyz], t: &Transform) -> XyzSet {
    vec.iter()
        .map(|p| {
            let e =
                |r: usize| -> i32 { p[0] * t[r][0] + p[1] * t[r][1] + p[2] * t[r][2] + t[r][3] };
            [e(0), e(1), e(2)]
        })
        .collect()
}

/// Multiply two transforms
fn mulm(a: &Transform, b: &Transform) -> Transform {
    let e = |r: usize, c: usize| -> i32 {
        a[r][0] * b[0][c] + a[r][1] * b[1][c] + a[r][2] * b[2][c] + a[r][3] * b[3][c]
    };
    [
        [e(0, 0), e(0, 1), e(0, 2), e(0, 3)],
        [e(1, 0), e(1, 1), e(1, 2), e(1, 3)],
        [e(2, 0), e(2, 1), e(2, 2), e(2, 3)],
        [e(3, 0), e(3, 1), e(3, 2), e(3, 3)],
    ]
}

/// Calculate inverse of a transform matrix
fn invert(a: &Transform) -> Transform {
    let mut b = [
        [a[0][0], a[1][0], a[2][0], 0],
        [a[0][1], a[1][1], a[2][1], 0],
        [a[0][2], a[1][2], a[2][2], 0],
        [0, 0, 0, 1],
    ];

    b[0][3] = -(b[0][0] * a[0][3] + b[0][1] * a[1][3] + b[0][2] * a[2][3]);
    b[1][3] = -(b[1][0] * a[0][3] + b[1][1] * a[1][3] + b[1][2] * a[2][3]);
    b[2][3] = -(b[2][0] * a[0][3] + b[2][1] * a[1][3] + b[2][2] * a[2][3]);

    b
}

fn solve(input: &str) -> (usize, i32) {
    let scans = parse(input);

    let combinations: Vec<(usize, usize)> = (0..scans.len())
        .map(|i1| ((i1 + 1)..scans.len()).map(move |i2| (i1, i2)))
        .flatten()
        .collect();

    let mappings: Vec<(usize, usize, Transform)> = combinations
        .par_iter()
        .filter_map(|(i1, i2)| check_rotations(&scans[*i1], &scans[*i2]).map(|tr| (*i1, *i2, tr)))
        .collect();

    let mut transforms = HashMap::<usize, Transform>::new();
    transforms.insert(0, [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]);

    /* Loop over transforms, insert mappings to view according to scanner 0 */
    while transforms.len() != scans.len() {
        for (dst, src, transform) in &mappings {
            if *dst == 0 {
                transforms.insert(*src, *transform);
            } else if let Some(oth) = transforms.get(dst) {
                let tr = mulm(oth, transform);
                transforms.insert(*src, tr);
            } else if let Some(oth) = transforms.get(src) {
                let inv = invert(transform);
                let tr = mulm(oth, &inv);
                transforms.insert(*dst, tr);
            }
        }
    }

    let mut result = XyzSet::new();
    for (i, v) in scans.iter().enumerate() {
        let res = transform(v, &transforms[&i]);
        result.extend(res.iter());
    }

    // Calculate max distance using offsets
    let mut dist = 0;
    for t1 in transforms.values() {
        for t2 in transforms.values() {
            dist = max(
                (t1[0][3] - t2[0][3]).abs()
                    + (t1[1][3] - t2[1][3]).abs()
                    + (t1[2][3] - t2[2][3]).abs(),
                dist,
            );
        }
    }

    (result.len(), dist)
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input));
}

#[test]
fn simple() {
    let input = include_str!("input-simple");
    assert_eq!(solve(input).0, 79);
}

#[test]
fn trivial() {
    let input = include_str!("input-trivial");
    assert_eq!(solve(input).0, 6);
}

#[test]
fn matrix() {
    let unit = [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]];
    assert_eq!(mulm(&unit, &unit), unit);
    let offset = [[1, 0, 0, 2], [0, 1, 0, 3], [0, 0, 1, 5], [0, 0, 0, 1]];
    assert_eq!(
        invert(&unit),
        [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1],]
    );

    assert_eq!(
        mulm(&offset, &offset),
        [[1, 0, 0, 4], [0, 1, 0, 6], [0, 0, 1, 10], [0, 0, 0, 1],]
    );
    assert_eq!(
        invert(&offset),
        [[1, 0, 0, -2], [0, 1, 0, -3], [0, 0, 1, -5], [0, 0, 0, 1],]
    );
}
