use std::collections::HashMap;

const ADDRESS: usize = 277678;

fn spiral(addr: usize) -> usize {
    let mut p = (0, 0);
    let mut dp = (0, -1);

    let mut values: HashMap<(isize, isize), usize> = HashMap::new();
    
    values.insert((0,0), 1);

    loop {
        if p.0 == p.1 || (p.0 < 0 && p.0 == -p.1) || (p.0 > 0 && p.0 == 1 - p.1) {
            dp = (-dp.1, dp.0);
        }
        p = (p.0 + dp.0, p.1 + dp.1);
        let val = 
            values.get(&(p.0 - 1, p.1)).unwrap_or(&0) +
            values.get(&(p.0 - 1, p.1 -1)).unwrap_or(&0) +
            values.get(&(p.0, p.1 - 1)).unwrap_or(&0) +
            values.get(&(p.0 + 1, p.1 - 1)).unwrap_or(&0) +
            values.get(&(p.0 + 1, p.1)).unwrap_or(&0) +
            values.get(&(p.0 + 1, p.1 + 1)).unwrap_or(&0) +
            values.get(&(p.0, p.1 + 1)).unwrap_or(&0) +
            values.get(&(p.0 - 1, p.1 + 1)).unwrap_or(&0);
        if val >= addr { return val; }
        values.insert(p, val);
    }
}

fn main() {
    println!("{}", spiral(ADDRESS));
}
