use regex::Regex;

#[derive(Debug, Clone)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

fn parse(input: &str) -> Vec<(String, Ingredient)> {
    let mut ingredients = Vec::new();
    let re = Regex::new(r"(\p{Alphabetic}+): capacity (-?\p{Digit}+), durability (-?\p{Digit}+), flavor (-?\p{Digit}+), texture (-?\p{Digit}+), calories (-?\p{Digit})").unwrap();
    for cap in input.lines().map(|line| re.captures(line).unwrap()) {
        ingredients.push((
            cap[1].to_string(),
            Ingredient {
                capacity: cap[2].parse::<i64>().unwrap(),
                durability: cap[3].parse::<i64>().unwrap(),
                flavor: cap[4].parse::<i64>().unwrap(),
                texture: cap[5].parse::<i64>().unwrap(),
                calories: cap[6].parse::<i64>().unwrap(),
            },
        ));
    }

    ingredients
}

fn score_recipe(recipe: &[(i64, &Ingredient)]) -> (i64, i64) {
    let capacity: i64 = std::cmp::max(0, recipe.iter().map(|(q, i)| q * i.capacity).sum());
    let durability: i64 = std::cmp::max(0, recipe.iter().map(|(q, i)| q * i.durability).sum());
    let flavor: i64 = std::cmp::max(0, recipe.iter().map(|(q, i)| q * i.flavor).sum());
    let texture: i64 = std::cmp::max(0, recipe.iter().map(|(q, i)| q * i.texture).sum());
    let calories: i64 = std::cmp::max(0, recipe.iter().map(|(q, i)| q * i.calories).sum());

    (capacity * durability * flavor * texture, calories)
}

fn solve(input: &str) -> (i64, i64) {
    let ingredients = parse(input);
    let mut max_score = 0;
    let mut max_score_500cal = 0;

    for a in 0..=100 {
        for b in 0..=(100 - a) {
            for c in 0..=(100 - a - b) {
                let d = 100 - a - b - c;
                let score = score_recipe(&[
                    (a, &ingredients[0].1),
                    (b, &ingredients[1].1),
                    (c, &ingredients[2].1),
                    (d, &ingredients[3].1),
                ]);
                if score.0 > max_score {
                    max_score = score.0;
                }
                if score.1 == 500 && score.0 > max_score_500cal {
                    max_score_500cal = score.0;
                }
            }
        }
    }
    (max_score, max_score_500cal)
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input));
}

#[test]
fn test_scoring() {
    let recipe = vec![
        Ingredient {
            capacity: -1,
            durability: -2,
            flavor: 6,
            texture: 3,
            calories: 8,
        },
        Ingredient {
            capacity: 2,
            durability: 3,
            flavor: -2,
            texture: -1,
            calories: 3,
        },
    ];
    let mut max_score = 0;
    for a in 0..=100 {
        let b = 100 - a;
        let score = score_recipe(&[(a, &recipe[0]), (b, &recipe[1])]).0;
        if score > max_score {
            max_score = score;
        }
    }
    assert_eq!(max_score, 62842880);
}
