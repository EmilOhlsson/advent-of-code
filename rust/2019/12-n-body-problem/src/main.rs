use regex::Regex;

fn sign_of(v: i32) -> i32 {
    (v > 0) as i32 - (v < 0) as i32
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    let re = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();
    input
        .lines()
        .map(|line| re.captures(line).unwrap())
        .map(|cap| {
            cap.iter()
                .skip(1)
                .map(|t| t.unwrap().as_str().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>()
}

fn solve(input: &str, steps: i32) -> i32 {
    let mut moons = parse(input);
    let mut velocities = vec![vec![0; 3]; moons.len()];
    for _ in 0..steps {
        for m1 in 0..moons.len() {
            for m2 in (m1 + 1)..moons.len() {
                for dim in 0..3 {
                    let puff = sign_of(moons[m2][dim] - moons[m1][dim]);
                    velocities[m1][dim] += puff;
                    velocities[m2][dim] -= puff;
                }
            }
            for dim in 0..3 {
                moons[m1][dim] += velocities[m1][dim];
            }
        }
    }

    let mut energy = 0;
    for m in 0..moons.len() {
        let pot = moons[m].iter().cloned().map(i32::abs).sum::<i32>();
        let kin = velocities[m].iter().cloned().map(i32::abs).sum::<i32>();
        energy += pot * kin;
    }

    energy
}

fn planar_repeat(moons: &[i32]) -> u64 {
    let p0 = moons.to_vec();
    let v0 = vec![0; 4];
    let mut pos = moons.to_vec();
    let mut vel = vec![0; 4];

    for i in 1u64.. {
        for m1 in 0..moons.len() {
            for m2 in (m1 + 1)..moons.len() {
                let puff = sign_of(pos[m2] - pos[m1]);
                vel[m1] += puff;
                vel[m2] -= puff;
            }
            pos[m1] += vel[m1];
        }

        if pos == p0 && vel == v0 {
            return i;
        }
    }
    panic!()
}

fn solve_v2(input: &str) -> u64 {
    use num::Integer;
    let moons = parse(input);

    let mut xyz = [0; 3];
    for dim in 0..3 {
        xyz[dim] = planar_repeat(&moons.iter().map(|m| m[dim]).collect::<Vec<i32>>());
    }
    xyz[0].lcm(&xyz[1]).lcm(&xyz[2])
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input, 1000));
    println!("{}", solve_v2(input));
}

#[test]
fn test0() {
    let input = include_str!("input-test0");
    assert_eq!(solve(input, 10), 179);
}

#[test]
fn test1() {
    let input = include_str!("input-test0");
    assert_eq!(solve_v2(input), 2772);
}

#[test]
fn test2() {
    let input = include_str!("input-test1");
    assert_eq!(solve_v2(input), 4686774924u64);
}
