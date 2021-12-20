use rayon::prelude::*;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

type Xyz = Vec<i32>;
type XSet = HashSet<i32>;
type XyzSet = HashSet<[i32; 3]>;
type XyzVec = Vec<Xyz>;

struct Range {
    val: i32,
    steps: i32,
    step: i32,
}

impl Range {
    fn new(a: i32, b: i32) -> Range {
        Range {
            val: a,
            steps: (b - a).abs(),
            step: (b > a) as i32 - (a > b) as i32,
        }
    }
}

impl Iterator for Range {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.steps <= 0 {
            None
        } else {
            let result = self.val;
            self.steps -= 1;
            self.val += self.step;
            Some(result)
        }
    }
}

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

fn get_dim(set: &XyzVec, dim: usize) -> HashSet<i32> {
    set.iter().map(|p| p[dim]).collect()
}

fn get_min_max(vs: &XSet) -> (i32, i32) {
    vs.iter().fold((i32::MAX, i32::MIN), |(lo, hi), v| {
        (min(lo, *v), max(hi, *v))
    })
}

/// Compare two sets of values by sliding over and finding best match.
/// Returns number of matches, and adjustment used for match
fn compare_dimension(a: &XSet, b: &XSet) -> (usize, i32) {
    let (lo_a, hi_a) = get_min_max(a);
    let (lo_b, hi_b) = get_min_max(b);
    let mut result = (0, 0);

    let lo = lo_a - hi_b;
    let hi = hi_a - lo_b;

    //println!("Range {}-{}", lo, hi);
    for offset in Range::new(lo, hi) {
        let adjusted = b.iter().map(|v| *v + offset).collect::<XSet>();
        let count = a.intersection(&adjusted).count();
        if count > result.0 {
            result = (count, offset);
        }
    }

    result
}

// The transform Ax + b = y can be written as A'x = y, using
// [ A   | b ] [ x ]   [ y ]
// [     | . ] [ . ] = [ . ]
// [ 0.. | 1 ] [ 1 ]   [ 1 ]
//
// Inverse of a ortgonal matrix is it's transpose, and that
// seem to be that case for swizzle matrices.
//
// So the inverse of the matrix above seems to be
//
// [ tr(A)  | -A^T * b ]
// [        |  ...     ]
// [ 0..    |     1    ]

/// This is a transform from one orientation to another.
/// (base dimension, modifier for revers/forward,
type Transform = [(usize, i32, i32); 3];
type Transform2 = [[i32; 4]; 4];

/// Compare two sets.
fn compare_sets(set_a: &XyzVec, set_b: &XyzVec) -> Option<Transform2> {
    let mut matches = 0;
    let mut transform: Transform2 = Default::default();
    transform[3][3] = 1;
    for dim_a in [0, 1, 2] {
        let xs_a = get_dim(set_a, dim_a);
        for dim_b in [0, 1, 2] {
            let xs_b = get_dim(set_b, dim_b);
            let (count, offset) = compare_dimension(&xs_a, &xs_b);
            if count >= 12 {
                // dim_a and dim_b(fw) matches, with offset adj
                transform[dim_a][dim_b] = 1;
                transform[dim_a][3] = offset;
                matches += 1;
                break;
            }

            // Revers set, and try that instead
            let xs_b = xs_b.iter().map(|&v| -v).collect::<XSet>();
            let (count, offset) = compare_dimension(&xs_a, &xs_b);
            if count >= 12 {
                // dim_a and dim_b(rev) matches with offset
                transform[dim_a][dim_b] = -1;
                transform[dim_a][3] = offset;
                matches += 1;
                break;
            }
        }
        // If one dimension doesn't have a match, no need to try the rest
        // This is just an optimization
        if matches == 0 {
            break;
        }
    }

    if matches == 3 {
        Some(transform)
    } else {
        None
    }
}

/// Better to use Affine transformation here likely: y = Ax, where x is the array [x y z 1]
/// and A us for example [[ A | b ]
///                       [ 0 | 1 ]]
///
fn transform(point: &[i32], tr: &Transform) -> [i32; 3] {
    [
        tr[0].2 + tr[0].1 * point[tr[0].0],
        tr[1].2 + tr[1].1 * point[tr[1].0],
        tr[2].2 + tr[2].1 * point[tr[2].0],
    ]
}

fn solve(input: &str) -> usize {
    let set = parse(input);

    // TODO: might need to keep track of inverse transforms
    let mut transforms = HashMap::<usize, Vec<(usize, Transform2)>>::new();

    for (a, beacons1) in set.iter().enumerate() {
        for (b, beacons2) in set.iter().enumerate().skip(a + 1) {
            //println!("Comparing {} and {}", a, b);
            if let Some(tr) = compare_sets(beacons1, beacons2) {
                transforms.entry(b).or_insert_with(Vec::new).push((a, tr));
                println!("{} and {} matches: {:?}", a, b, tr);

                // TODO: Need to keep track of transformations, and then map
                // back into something relative to first set.
            }
        }
    }

    // TODO: Maybe build a transform path to view of scanner[0]

    // Store points from the view of scanner[0]
    let mut beacons = XyzSet::new();
    for p in &set[0] {
        beacons.insert([p[0], p[1], p[2]]);
    }
    for (i, bs) in set.iter().enumerate() {
        // TODO find transformation path to view of scanner 0, using transforms map above
    }

    beacons.len()
}

fn solve_rayon(input: &str) -> usize {
    let vec = parse(input);
    let combinations: Vec<(usize, usize)> = (0..vec.len())
        .map(|i1| ((i1 + 1)..vec.len()).map(move |i2| (i1, i2)))
        .flatten()
        .collect();

    let transforms: Vec<(usize, usize, Transform2)> = combinations
        .par_iter()
        .filter_map(|(i1, i2)| compare_sets(&vec[*i1], &vec[*i2]).map(|tr| (*i1, *i2, tr)))
        .collect();
    println!("Transforms: {:?}", transforms);

    todo!()
}

fn main() {
    let input = include_str!("input");
    //println!("{}", solve(input));
    println!("{}", solve_rayon(input));
}

#[test]
fn test_simple() {
    let input = include_str!("input-simple");
    assert_eq!(solve_rayon(input), 79);
}
