use std::cmp::{max, min};
use std::collections::HashMap;

use coding_challenge_utils::coord::Cartesian;
use regex::Regex;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Grid {
    Clay,
    Water,
    WaterFlow
}

fn _print_grid(grid: &HashMap<Cartesian, Grid>, (xr, yr): ((i32, i32), (i32, i32))) {
    for y in (yr.0 - 1)..=(yr.1 + 1) {
        print!("{:3} ",y);
        for x in (xr.0 - 1)..=(xr.1 + 1){
            if (500, 0) == (x, y) {
                print!("+");
            } else {
            print!(
                "{}",
                match grid.get(&Cartesian::new(x, y)) {
                    Some(Grid::Clay) => '#',
                    Some(Grid::Water) => '=',
                    Some(Grid::WaterFlow) => '~',
                    None => '.',
                }
            );
            }
        }
        println!();
    }
    println!();
}

fn flow(grid: &mut HashMap<Cartesian, Grid>,mut pos: Cartesian, y_lim: i32, (xr, yr): ((i32, i32), (i32, i32))) {
    let mut stop = false;
    // Flow down until wall or still water
    while match *grid.entry(pos).or_insert(Grid::WaterFlow) {
        Grid::Clay | Grid::Water => false,
        _ => true,
    } {
        pos += Cartesian::new(0, 1);
        if pos.y > y_lim {
            return;
        }
    }

    // Fill up
    while !stop {
        pos += Cartesian::new(0, -1);

        let mut pos_l = pos + Cartesian::new(-1, 0);
        let mut pos_ld = pos + Cartesian::new(-1, 1);
        let mut pos_lpd = pos + Cartesian::new(0, 1);
        let mut pos_lp = pos;

        let mut pos_r = pos + Cartesian::new(1, 0);
        let mut pos_rd = pos + Cartesian::new(1, 1);
        let mut pos_rpd = pos + Cartesian::new(0, 1);
        let mut pos_rp = pos;
        
        // Fill left while don't hit a wall or not a flow point
        loop {
            let c = *grid.entry(pos_l).or_insert(Grid::WaterFlow);
            let cd = *grid.entry(pos_ld).or_insert(Grid::WaterFlow);
            let pd = *grid.entry(pos_lpd).or_insert(Grid::WaterFlow);

            if cd == Grid::WaterFlow && pd == Grid::Clay {
                stop = true;
                flow(grid, pos_l, y_lim, (xr, yr));
                break;
            }
            if pd == Grid::WaterFlow {
                grid.insert(pos_lpd, Grid::Water);
            }

            if c == Grid::Clay {
                break;
            }

            pos_l += Cartesian::new(-1, 0);
            pos_ld += Cartesian::new(-1, 0);
            pos_lpd += Cartesian::new(-1, 0);
        }

        // Fill right while don't hit a wall or not a flow point
        loop {
            let c = *grid.entry(pos_r).or_insert(Grid::WaterFlow);
            let cd = *grid.entry(pos_rd).or_insert(Grid::WaterFlow);
            let pd = *grid.entry(pos_rpd).or_insert(Grid::WaterFlow);

            if cd == Grid::WaterFlow && pd == Grid::Clay {
                stop = true;
                flow(grid, pos_r, y_lim, (xr, yr));
                break;
            }
            if pd == Grid::WaterFlow {
                grid.insert(pos_rpd, Grid::Water);
            }

            if c == Grid::Clay {
                break;
            }

            pos_r += Cartesian::new(1, 0);
            pos_rd += Cartesian::new(1, 0);
            pos_rpd += Cartesian::new(1, 0);
        }

        if !stop {
            println!("Filling on level {}", pos.y + 1);
            for x in pos_lpd.x..=pos_rpd.x {
                *grid.entry(Cartesian::new(x, pos.y)).or_insert(Grid::Water) = Grid::Water;
            }
        }
    }
}

fn solve(input: &str) -> (usize, usize) {
    let mut grid: HashMap<Cartesian, Grid> = HashMap::new();
    let re = Regex::new(r"(?m)^([xy])=(\d+), ([xy])=(\d+)\.\.(\d+)$").unwrap();
    let mut yr = (0, 0);
    let mut xr = (500, 500);

    for cap in re.captures_iter(input) {
        let lim = cap[2].parse::<i32>().unwrap();
        let lower = cap[4].parse::<i32>().unwrap();
        let upper = cap[5].parse::<i32>().unwrap();
        match &cap[1] {
            "x" => {
                xr = (min(xr.0, lim), max(xr.1, lim));
                yr = (min(yr.0, lower), max(yr.1, upper));
                for y in lower..=upper {
                    grid.insert(Cartesian::new(lim, y), Grid::Clay);
                }
            }
            "y" => {
                xr = (min(xr.0, lower), max(xr.1, upper));
                yr = (min(yr.0, lim), max(yr.1, lim));
                for x in lower..=upper {
                    grid.insert(Cartesian::new(x, lim), Grid::Clay);
                }
            }
            _ => panic!(),
        }
    }

    flow(&mut grid, Cartesian::new(500, 1), yr.1, (xr, yr));
    _print_grid(&grid, (xr, yr));

    (grid.values().filter(|g| *g == &Grid::Water || *g == &Grid::WaterFlow).count(),
    grid.values().filter(|g| *g == &Grid::Water).count())
}

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", solve(input));
}

#[test]
fn test() {
    let input = include_str!("input-simple.txt");
    assert_eq!(solve(input), (57, 29));
}
