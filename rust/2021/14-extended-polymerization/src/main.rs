use std::collections::HashMap;

const LETTERS: usize = 'Z' as usize - 'A' as usize + 1;

#[derive(Clone, Copy, Debug, Default)]
struct Counts([u64; LETTERS]);

impl Counts {
    fn iter(&self) -> std::slice::Iter<'_, u64> {
        self.0.iter()
    }

    fn at(&mut self, i: usize) -> &mut u64 {
        &mut self.0[i]
    }

    fn at_char(&mut self, ch: char) -> &mut u64 {
        &mut self.0[ch as usize - 'A' as usize]
    }
}

type Cache = HashMap<(u32, [char; 2]), Counts>;
type Map = HashMap<[char; 2], char>;

fn get_counts(map: &Map, cache: &mut Cache, levels: u32, pat: [char; 2]) -> Counts {
    if let Some(counts) = cache.get(&(levels, pat)) {
        *counts
    } else {
        let counts = if levels == 0 {
            Default::default()
        } else if let Some(ch) = map.get(&pat) {
            let mut counts: Counts = Default::default();
            *counts.at_char(*ch) = 1;
            let left = get_counts(map, cache, levels - 1, [pat[0], *ch]);
            let right = get_counts(map, cache, levels - 1, [*ch, pat[1]]);
            for (i, (a, b)) in left.iter().zip(right.iter()).enumerate() {
                *counts.at(i) += *a + *b;
            }
            counts
        } else {
            let mut counts: Counts = Default::default();
            *counts.at_char(pat[0]) += 1;
            *counts.at_char(pat[1]) += 1;
            counts
        };
        cache.insert((levels, pat), counts);
        counts
    }
}

fn solve(input: &str, steps: u32) -> u64 {
    let mut lines = input.lines();
    let polymer = lines.next().unwrap().chars().collect::<Vec<char>>();
    let translations = lines
        .skip(1)
        .map(|line| {
            let mut toks = line.split(" -> ");
            let a = toks.next().unwrap().chars().collect::<Vec<char>>();
            let a = [a[0], a[1]];
            let b = toks.next().unwrap().chars().next().unwrap();
            (a, b)
        })
        .collect::<Map>();

    let mut counts: Counts = Default::default();
    for ch in &polymer {
        *counts.at_char(*ch) += 1;
    }

    let mut cache = Cache::new();
    for win in polymer.windows(2) {
        let cs = get_counts(&translations, &mut cache, steps, [win[0], win[1]]);
        for (i, count) in cs.iter().enumerate() {
            //counts[i] += count;
            *counts.at(i) += count;
        }
    }

    /* Which is most/least abundant? */
    let (min, max) = counts
        .iter()
        .filter(|&v| *v != 0)
        .fold((u64::MAX, 0), |(min, max), count| {
            (std::cmp::min(*count, min), std::cmp::max(*count, max))
        });

    max - min
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input, 10));
    println!("{}", solve(input, 40));
}

#[test]
fn test_p1() {
    let input = include_str!("input-simple");
    assert_eq!(solve(input, 10), 1588);
}

#[test]
fn test_p2() {
    let input = include_str!("input-simple");
    assert_eq!(solve(input, 40), 2188189693529);
}
