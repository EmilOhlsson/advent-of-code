extern crate regex;

use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Guard {
    asleep: bool,
    fell_asleep: u32,
    sleep_minutes: u32,
    sleepings: Vec<(u32, u32)>,
}

fn solve(input: &str) -> (usize, usize) {
    let re_begin = Regex::new(r"\[\d+-\d+-\d+ \d+:(\d\d)\] Guard #(\d+) begins shift").unwrap();
    let re_falls_asleep = Regex::new(r"\[\d+-\d+-\d+ \d+:(\d\d)\] falls asleep").unwrap();
    let re_wake_up = Regex::new(r"\[\d+-\d+-\d+ \d+:(\d\d)\] wakes up").unwrap();

    let mut guard = 0;
    let mut guards: HashMap<usize, Guard> = HashMap::new();

    for line in input.lines() {
        if let Some(cpt) = re_begin.captures(line) {
            let t = cpt[1].parse::<u32>().unwrap();
            let new_guard = cpt[2].parse::<usize>().unwrap();

            // If guard does not exist, add the guard
            if !guards.contains_key(&new_guard) {
                guards.insert(
                    new_guard,
                    Guard {
                        asleep: false,
                        fell_asleep: t,
                        sleep_minutes: 0,
                        sleepings: Vec::new(),
                    },
                );
            }
            guard = new_guard;
        } else if let Some(cpt) = re_falls_asleep.captures(line) {
            let t = cpt[1].parse::<u32>().unwrap();
            let g = guards.get_mut(&guard).unwrap();
            g.asleep = true;
            g.fell_asleep = t;
        } else if let Some(cpt) = re_wake_up.captures(line) {
            let t = cpt[1].parse::<u32>().unwrap();
            let g = guards.get_mut(&guard).unwrap();
            g.asleep = false;
            let minutes_of_sleep = t - g.fell_asleep;
            g.sleep_minutes += minutes_of_sleep;
            g.sleepings.push((g.fell_asleep, minutes_of_sleep));
        } else {
            panic!("Does not recognize: {}", line);
        }
    }

    let sleepiest = guards.iter().max_by_key(|(_, g)| g.sleep_minutes).unwrap();
    let minute = (0..60)
        .max_by_key(|&m| {
            sleepiest
                .1
                .sleepings
                .iter()
                .fold(0, |acc, &(start, length)| {
                    acc + (m >= start && m < start + length) as u32
                })
        }).unwrap();

    let mut min = 0;
    let mut id = 0;
    let mut best_slept = 0;
    for (i, g) in &guards {
        for m in 0..60 {
            let times_slept = g.sleepings.iter().fold(0, |acc, &(start, length)| {
                acc + (m >= start && m < start + length) as u32
            });
            if times_slept > best_slept {
                best_slept = times_slept;
                id = *i;
                min = m as usize;
            }
        }

    }

    (sleepiest.0 * minute as usize, min * id)
}

fn main() {
    let input = include_str!("input.txt");
    let solution = solve(input);
    println!("{}", solution.0);
    println!("{}", solution.1);
}
