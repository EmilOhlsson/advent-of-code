type Map = Vec<Vec<char>>;

fn parse(input: &str) -> Map {
    input
        .lines()
        .map(move |line| line.chars().collect())
        .collect()
}

fn tick(map: &Map) -> Map {
    let mut result = vec![vec!['.'; map[0].len()]; map.len()];
    for (r, row) in map.iter().enumerate() {
        for (c, _) in row.iter().enumerate().filter(|(_, &ch)| ch == '>') {
            let c_next = (c + 1) % row.len();
            if row[c_next] == '.' {
                result[r][c_next] = '>';
            } else {
                result[r][c] = '>';
            }
        }
    }

    for (r, row) in map.iter().enumerate() {
        for (c, _) in row.iter().enumerate().filter(|(_, &ch)| ch == 'v') {
            let r_next = (r + 1) % map.len();
            if map[r_next][c] != 'v' && result[r_next][c] == '.' {
                result[r_next][c] = 'v';
            } else {
                result[r][c] = 'v';
            }
        }
    }

    result
}

fn solve(input: &str) -> usize {
    let mut map = parse(input);
    for step in 1.. {
        let map_next = tick(&map);
        if map == map_next {
            return step;
        }
        map = map_next;
    }
    panic!("Never stops!");
}

fn _print_map(map: &Map) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input));
}

#[test]
fn simple() {
    let input = include_str!("input-simple");
    assert_eq!(solve(input), 58);
}

#[test]
fn right() {
    let mut gen = parse("..>>>>..");

    gen = tick(&gen);
    assert_eq!(gen, parse("..>>>.>."));

    gen = tick(&gen);
    assert_eq!(gen, parse("..>>.>.>"));

    gen = tick(&gen);
    assert_eq!(gen, parse(">.>.>.>."));

    gen = tick(&gen);
    assert_eq!(gen, parse(".>.>.>.>"));
}
