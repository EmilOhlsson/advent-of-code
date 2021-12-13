use std::cmp::{max, min};
use std::collections::HashSet;

fn solve(input: &str) {
    let mut reading_dots = true;
    let mut dots = HashSet::<(i32, i32)>::new();
    let mut folds = Vec::<(char, i32)>::new();
    for line in input.lines() {
        reading_dots = reading_dots && !line.is_empty();
        if reading_dots {
            let mut toks = line.split(',');
            let x = toks.next().unwrap().parse::<i32>().unwrap();
            let y = toks.next().unwrap().parse::<i32>().unwrap();
            dots.insert((x, y));
        } else if !line.is_empty() {
            let mut toks = line.split_whitespace().nth(2).unwrap().split('=');
            let axis = toks.next().unwrap().chars().next().unwrap();
            let line = toks.next().unwrap().parse::<i32>().unwrap();
            folds.push((axis, line));
        }
    }

    for (axis, fl) in &folds {
        let dots_new = if *axis == 'x' {
            dots.iter()
                .map(|(x, y)| (if x > fl { *fl - (x - *fl) } else { *x }, *y))
                .collect()
        } else {
            dots.iter()
                .map(|(x, y)| (*x, if y > fl { *fl - (y - *fl) } else { *y }))
                .collect()
        };
        dots = dots_new;
        println!("{}", dots.len());
    }

    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;
    for (x, y) in &dots {
        x_min = min(*x, x_min);
        x_max = max(*x, x_max);
        y_min = min(*y, y_min);
        y_max = max(*y, y_max);
    }

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            print!("{}", if dots.contains(&(x, y)) { '#' } else { ' ' });
        }
        println!();
    }
}

fn main() {
    let input = include_str!("input");
    solve(input);
}
