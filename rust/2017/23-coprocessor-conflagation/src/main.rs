fn translated() -> isize {
    let mut b = 81 * 100 + 100_000;
    let c = b + 17_000;
    let mut h = 0;
    let mut g;

    while {
        let mut f = 1;
        let d = 2;
        for dp in (d*d)..b { 
            if b % dp == 0 {
                f = 0;
                break;
            }
        }


        if f == 0 {
            h += 1;
        }

        g = b - c;
        b += 17;
        g != 0
    } {}
    h
}

fn main() {
    println!("{}", translated());
}
