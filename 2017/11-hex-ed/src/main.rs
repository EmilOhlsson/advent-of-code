use std::num::ParseIntError;
use std::str::FromStr;
use std::ops::Add;

#[derive(Debug, Clone)]
struct HCord {
    x: isize,
    y: isize,
    z: isize,
}

impl HCord {
    fn new(x: isize, y: isize, z: isize) -> HCord {
        HCord {x, y , z}
    }

    fn distance(&self, other: &HCord) -> isize {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) / 2
    }
}

impl Add for HCord {
    type Output = HCord;

    fn add(self, other: HCord) -> HCord {
        HCord {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl FromStr for HCord {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<HCord, Self::Err> {
        match s {
            "se" => Ok(HCord::new(1, -1, 0)),
            "ne" => Ok(HCord::new(1, 0, -1)),
            "n" => Ok(HCord::new(0, 1, -1)),
            "nw" => Ok(HCord::new(-1, 1, 0)),
            "sw" => Ok(HCord::new(-1, 0, 1)),
            "s" => Ok(HCord::new(0, -1, 1)),
            _ => panic!("Unexpected: {}", s),
        }
    }
}

fn hex_distance(direction_str: &str) -> isize {
    let origin = HCord::new(0, 0, 0);
    let mut furthest = HCord::new(0,0,0);
    let destination: HCord = direction_str.split(',')
        .map(|s| s.parse::<HCord>().unwrap())
        .fold(HCord::new(0, 0, 0), |a, s| {
            if a.distance(&origin) > furthest.distance(&origin) {
                furthest = a.clone();
            }
            a + s
        });
    println!("furthest: {}", furthest.distance(&origin));
    return destination.distance(&origin);
}

fn main() {
    let hdist = hex_distance(include_str!("input").trim());
    println!("{}", hdist);
}

#[test]
fn test_inputs() {
    assert_eq!(hex_distance("ne,ne,ne"), 3);
    assert_eq!(hex_distance("ne,ne,sw,sw"), 0);
    assert_eq!(hex_distance("ne,ne,s,s"), 2);
    assert_eq!(hex_distance("se,sw,se,sw,sw"), 3);
}
