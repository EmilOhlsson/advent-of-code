use std::collections::HashSet;

fn main() {
    let mut r = vec![0; 6];
    let mut seen: HashSet<usize> = HashSet::new();
    loop {
        r[3] = r[4] | 65536;
        r[4] = 10283511;
        loop {
            r[1] = r[3] & 255;
            r[4] += r[1];
            r[4] &= 16777215;
            r[4] *= 65899;
            r[4] &= 16777215;
            if 256 > r[3] {
                if seen.insert(r[4]) {
                    println!("{}", r[4]);
                }
                break;
            }
            r[3] /= 256;
        }
    }
}
