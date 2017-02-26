use std::fs::File;
use std::io::{Read, BufReader};

fn decompress(cnt: &str) -> usize {
    let mut next = String::from(cnt);
    let mut size: usize = 0;

    loop {
        let toks: Vec<String> = next.splitn(2, '(').map(|x| String::from(x)).collect();
        size += toks[0].len();
        if toks.len() < 2 {
            break;
        }
        
        let itoks: Vec<String> = toks[1].splitn(2, ')').map(|x| String::from(x)).collect();
        let factors: Vec<usize> = itoks[0].split('x').filter_map(|s| s.parse::<usize>().ok()).collect();

        let (exp, trail) = itoks[1].split_at(factors[0]);
        next = String::from(trail);

        let intd = decompress(exp);
        for _ in 0..factors[1] {
            size += intd;
        }
    }

    return size;
}

#[test]
fn simple_tests() {
    assert_eq!(decompress("ADVENT"), "ADVENT");
    assert_eq!(decompress("ADVENT").len(), 6);
    assert_eq!(decompress("A(1x5)BC"), "ABBBBBC");
    assert_eq!(decompress("A(1x5)BC").len(), 7);
    assert_eq!(decompress("(3x3)XYZ"), "XYZXYZXYZ");
    assert_eq!(decompress("(3x3)XYZ").len(), 9);
    assert_eq!(decompress("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
    assert_eq!(decompress("A(2x2)BCD(2x2)EFG").len(), 11);
    assert_eq!(decompress("(6x1)(1x3)A"), "(1x3)A");
    assert_eq!(decompress("(6x1)(1x3)A").len(), 6);
    assert_eq!(decompress("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
    assert_eq!(decompress("X(8x2)(3x3)ABCY").len(), 18);
}

fn main() {
    let mut buffer = String::new();
    let mut input = match File::open("input.txt") {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("Unable to open input.txt: {}", e),
    };
    if let Ok(_) = input.read_to_string(&mut buffer) {
        let decompressed = decompress(&buffer.trim());
        println!("decompressed size: {}", decompressed);
    }
}
