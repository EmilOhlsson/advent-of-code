use std::collections::HashMap;

static INITIAL_STATE: &str = "#......##...#.#.###.#.##..##.#.....##....#.#.##.##.#..#.##........####.###.###.##..#....#...###.##";
static INITIAL_STATE_SIMPLE: &str = "#..#.#..##......###...###";

fn get_at(state: &Vec<bool>, index: i64) -> bool {
    if index < 0 {
        false
    } else {
        *state.get(index as usize).unwrap_or(&false)
    }
}

fn pot_str(pots: &Vec<bool>) -> String {
    pots.iter().map(|p| if *p { '#' } else { '.' }).collect::<String>()
}

fn linesum(pots: &Vec<bool>, offset: i64) -> i64 {
    pots.iter().enumerate().map(|(i, p) | if *p { i as i64 + offset } else { 0 }).sum()
}

fn solve(input: &str, inital_state: &str, generations: i64) -> i64 {
    let translations = input
        .lines()
        .map(|l| {
            let ts = l.split_whitespace().collect::<Vec<_>>();
            (
                ts[0].chars().map(|ch| ch == '#').collect::<Vec<bool>>(),
                ts[2] == "#",
            )
        })
        .collect::<HashMap<Vec<bool>, bool>>();

    let block = 5;
    let mut index_offset = 0;
    let mut state = inital_state
        .chars()
        .map(|ch| ch == '#')
        .collect::<Vec<bool>>();
    let mut lsum = linesum(&state, 0);

    println!("{}  {}", lsum,  pot_str(&state));
    for _ in 0..generations {
        let mut new_state = Vec::new();
        for i in 0..(state.len() as i64 + block) {
            /* Build local chunk */
            let mut chunk = Vec::new();
            for j in (i - block + 1)..=i {
                chunk.push(get_at(&state, j));
            }

            new_state.push(*translations.get(&chunk).unwrap_or(&false));
        }
        index_offset -= block / 2;
        let new_lsum = linesum(&new_state, index_offset);

        println!("{} {} :::: {}", new_lsum, new_lsum - lsum, pot_str(&new_state));
        lsum = new_lsum;
        state = new_state;
    }

    lsum
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", solve(input, INITIAL_STATE, 20));
    println!("{}", solve(input, INITIAL_STATE, 1000) + 75 * (50000000000 - 1000));
}

#[test]
fn test() {
    assert_eq!(
        solve(include_str!("input-simple.txt"), INITIAL_STATE_SIMPLE, 20),
        325
    );
}
