use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn solve_v2(input: &str) -> (usize, usize) {
    let mut lines = input.lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect::<Vec<usize>>();

    let mut row = 0;
    let mut board_number = 0;
    let mut boards = HashMap::<(usize, usize, usize), usize>::new();
    let mut number_to_board_position = HashMap::<usize, Vec<(usize, usize, usize)>>::new();

    /* Build bingo boards and mapping */
    for line in lines.map(str::trim) {
        if line.is_empty() {
            board_number += 1;
            row = 0;
        } else {
            /* Add row of number to board, and create mapping */
            for (col, num) in line
                .split_whitespace()
                .map(str::parse::<usize>)
                .map(Result::unwrap)
                .enumerate()
            {
                boards.insert((board_number, row, col), num);
                let entry = number_to_board_position.entry(num).or_insert_with(Vec::new);
                entry.push((board_number, row, col));
            }
            row += 1;
        }
    }

    let mut marked = HashSet::<(usize, usize, usize)>::new();
    let mut winning_boards = HashSet::<usize>::new();
    let board_size = 5;
    let mut part1 = None;

    /* Play bingo! */
    for number in &numbers {
        for &(board, row, col) in &number_to_board_position[number] {
            marked.insert((board, row, col));

            // Check for win
            let win = (0..board_size).all(|r| marked.contains(&(board, r, col)))
                || (0..board_size).all(|c| marked.contains(&(board, row, c)));
            if win {
                winning_boards.insert(board);
                if winning_boards.len() == board_number || part1.is_none() {
                    let unmarked_sum = (0..board_size)
                        .cartesian_product(0..board_size)
                        .filter(|(r, c)| !marked.contains(&(board, *r, *c)))
                        .map(|(r, c)| boards[&(board, r, c)])
                        .sum::<usize>();

                    if let Some(part1) = part1 {
                        return (part1, unmarked_sum * number);
                    } else {
                        part1 = Some(unmarked_sum * number);
                    }
                }
            }
        }
    }

    panic!("No one won!")
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve_v2(input));
}

#[test]
fn test_simple() {
    let input = include_str!("input-simple");
    assert_eq!(solve_v2(input), (4512, 1924));
}
