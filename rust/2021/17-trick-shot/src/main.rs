use std::cmp::max;

type Xy = (i32, i32);

struct Trajectory {
    pos: Xy,
    vel: Xy,

    x_range: Xy,
    y_range: Xy,
}

impl Trajectory {
    fn new(vel: Xy, x_range: Xy, y_range: Xy) -> Trajectory {
        Trajectory {
            pos: (0, 0),
            vel,
            x_range,
            y_range,
        }
    }

    fn tick(&mut self) {
        self.pos = (self.pos.0 + self.vel.0, self.pos.1 + self.vel.1);
        self.vel = (max(self.vel.0 - 1, 0), self.vel.1 - 1);
    }

    fn within_target(&self) -> bool {
        self.pos.0 >= self.x_range.0
            && self.pos.0 <= self.x_range.1
            && self.pos.1 >= self.y_range.0
            && self.pos.1 <= self.y_range.1
    }

    fn missed_target(&self) -> bool {
        // Missed when passed or fallen below target
        self.pos.0 > self.x_range.1 || self.pos.1 < self.y_range.0
    }

    fn get_max(&mut self) -> Option<i32> {
        let mut max_y = 0;
        loop {
            self.tick();
            max_y = max(max_y, self.pos.1);
            if self.missed_target() {
                return None;
            } else if self.within_target() {
                return Some(max_y);
            }
        }
    }
}

fn solve(x_lo: i32, x_hi: i32, y_lo: i32, y_hi: i32) -> (i32, i32) {
    // minimum x-speed must be something that at least reaches target. How far
    // a given initial speed, n, reaches is x_{n} + x_{n-1} + ... 1, which is
    // the same as the nth triangle number, given by n * (n + 1) / 2. A simple
    // lower bound can be set to (2 * x_lo).sqrt()
    // Upper bound for x speed is x_hi + 1
    let x_min = (2.0 * x_lo as f32).sqrt() as i32;
    let mut y_max = 0;
    let mut hits = 0;
    for y in -1_000..1_000 {
        for x in x_min..=x_hi {
            let mut traj = Trajectory::new((x, y), (x_lo, x_hi), (y_lo, y_hi));
            if let Some(traj_max) = traj.get_max() {
                if traj_max > y_max {
                    y_max = traj_max;
                }
                hits += 1;
            }
        }
    }
    (y_max, hits)
}

fn main() {
    println!("{:?}", solve(29, 73, -248, -194));
}

#[test]
fn test_traj() {
    let mut traj = Trajectory::new((6, 9), (20, 30), (-10, -5));
    assert_eq!(traj.get_max(), Some(45));
}

#[test]
fn test() {
    assert_eq!(solve(20, 30, -10, -5), (45, 112));
}
