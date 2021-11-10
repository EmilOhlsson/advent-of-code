use regex::Regex;
use std::collections::HashMap;

struct Deer {
    speed: u32,
    running_capacity: u32,
    resting_capacity: u32,
    time_resting: u32,
    time_running: u32,
    is_running: bool,
    distance: u32,
    points: u32,
}

impl Deer {
    fn new(speed: u32, running_capacity: u32, resting_capacity: u32) -> Self {
        Deer {
            speed,
            running_capacity,
            resting_capacity,
            time_resting: 0,
            time_running: 0,
            is_running: true,
            distance: 0,
            points: 0,
        }
    }
}

fn parse(input: &str) -> HashMap<String, Deer> {
    let mut deers = HashMap::<String, Deer>::new();
    let re = Regex::new(r"(\p{Alphabetic}+) can fly (\p{Digit}+) km/s for (\p{Digit}+) seconds, but then must rest for (\p{Digit}+) seconds.").unwrap();
    for cap in input.lines().map(|l| re.captures(l).unwrap()) {
        deers.insert(
            cap[1].to_string(),
            Deer::new(
                cap[2].parse::<u32>().unwrap(),
                cap[3].parse::<u32>().unwrap(),
                cap[4].parse::<u32>().unwrap(),
            ),
        );
    }
    deers
}

fn solve(input: &str, duration: u32) -> (u32, u32) {
    let mut deers = parse(input);

    for _ in 0..duration {
        deers.values_mut().for_each(|deer| {
            if deer.is_running {
                deer.distance += deer.speed;
                deer.time_running += 1;
                if deer.time_running >= deer.running_capacity {
                    deer.is_running = false;
                    deer.time_running = 0;
                }
            } else {
                deer.time_resting += 1;
                if deer.time_resting >= deer.resting_capacity {
                    deer.is_running = true;
                    deer.time_resting = 0;
                }
            }
        });
        let max_distance = deers.values().map(|deer| deer.distance).max().unwrap();
        deers.values_mut().for_each(|deer| {
            if deer.distance == max_distance {
                deer.points += 1
            }
        });
    }
    (
        deers.values().map(|d| d.distance).max().unwrap(),
        deers.values().map(|d| d.points).max().unwrap(),
    )
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input, 2503));
}
