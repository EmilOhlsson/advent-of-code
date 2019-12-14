use std::cmp::max;

use std::collections::HashMap;
type Recipie = HashMap<String, (i64, Vec<(i64, String)>)>;
type Stockpile = HashMap<String, i64>;

fn get_ore_amount(
    reciepie: &Recipie,
    stockpile: &mut Stockpile,
    ingredient: &str,
    amount: i64,
) -> i64 {
    let (produced, components) = reciepie.get(ingredient).unwrap();

    // Check how much is actually neeeded
    let available = stockpile.entry(ingredient.to_string()).or_insert(0);
    let needed = max(0, amount - *available);

    // How many times do we need to run reaction;
    let multiple = needed / produced + (needed % produced != 0) as i64;
    let excess = multiple * produced - amount;
    *available += excess;

    let mut sum = 0;
    for component in components {
        sum += if component.1 == "ORE" {
            component.0 * multiple
        } else {
            get_ore_amount(reciepie, stockpile, &component.1, component.0 * multiple)
        };
    }
    sum
}

fn solve(input: &str) -> (i64, i64) {
    let mut recipie = Recipie::new();

    // Parse input
    for line in input.lines() {
        let mut tok = line.split("=>");
        let ing = tok.next().unwrap().trim();
        let out = tok.next().unwrap().trim();
        let mut out_it = out.split_whitespace();
        let out_quant = out_it.next().unwrap().parse::<i64>().unwrap();
        let out_name = out_it.next().unwrap().to_string();

        recipie.insert(
            out_name,
            (
                out_quant,
                ing.split(',')
                    .map(str::trim)
                    .map(str::split_whitespace)
                    .map(|mut ts| {
                        let quant = ts.next().unwrap().parse::<i64>().unwrap();
                        let name = ts.next().unwrap().to_string();
                        (quant, name)
                    })
                    .collect::<Vec<(i64, String)>>(),
            ),
        );
    }
    let part1 = get_ore_amount(&recipie, &mut HashMap::new(), "FUEL", 1);

    // Binary search for part two. Start by estimating a rough interval
    let trillion = 1_000_000_000_000i64;
    let mut guess_low = trillion / part1;
    let mut guess_high = guess_low * 2;
    let mut part2 = -1;
    while guess_low < guess_high {
        part2 = (guess_low + guess_high) / 2;
        let actual = get_ore_amount(&recipie, &mut HashMap::new(), "FUEL", part2);
        if actual > trillion {
            guess_high = part2 - 1;
        } else {
            guess_low = part2 + 1;
        }
    }

    (part1, part2)
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input));
}

#[test]
fn test_multiple() {
    assert_eq!(8 / 4 + (8 % 4 != 0) as i64, 2);
    assert_eq!(9 / 4 + (9 % 4 != 0) as i64, 3);
}

#[test]
fn test_simple() {
    let input = include_str!("input-test-simple");
    assert_eq!(solve(input).0, 5);
}

#[test]
fn test0() {
    let input = include_str!("input-test0");
    assert_eq!(solve(input).0, 31);
}
