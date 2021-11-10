use regex::Regex;
use coding_challenge_utils::coord::Cartesian as P;
use itertools::Itertools;

fn solve(input: &str) {
    let re = Regex::new(r"position=<(.*)> velocity=<(.*)>").unwrap();
    let mut points = input.lines().map(|l| {
        let capt = re.captures(l).unwrap();
        (capt[1].parse::<P>().unwrap(), capt[2].parse::<P>().unwrap())
    }).collect::<Vec<(P, P)>>();

    for i in 0.. {
        let mut points_prev = points.clone();
        for (p, v) in points.iter_mut() {
            p.x += v.x;
            p.y += v.y;
        }
        let bbox_x_prev = points_prev.iter().map(|(p, _)| p.x).minmax().into_option().unwrap();
        let bbox_y_prev = points_prev.iter().map(|(p, _)| p.y).minmax().into_option().unwrap();

        let bbox_x = points.iter().map(|(p, _)| p.x).minmax().into_option().unwrap();
        let bbox_y= points.iter().map(|(p, _)| p.y).minmax().into_option().unwrap();

        let bbox_size_prev = (bbox_x_prev.1 - bbox_x_prev.0) as u64* (bbox_y_prev.1 - bbox_y_prev.0) as u64;
        let bbox_size = (bbox_x.1 - bbox_x.0) as u64 * (bbox_y.1 - bbox_y.0) as u64;
        println!("{}: {}, {}",i, bbox_size, bbox_size_prev);
        if bbox_size > bbox_size_prev {
            for y in bbox_y_prev.0..=bbox_y_prev.1 {
                for x in bbox_x_prev.0..=bbox_x_prev.1 {
                    let mut found = false;
                    for (p, _) in &points_prev {
                        if p.x == x && p.y == y { 
                            found = true;
                        }
                    }
                    if found {
                        print!("#");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }
            break;
        }
    }
}

fn main() {
    let test_input = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
    solve(test_input);

    let input = include_str!("input");
    solve(input);
}
