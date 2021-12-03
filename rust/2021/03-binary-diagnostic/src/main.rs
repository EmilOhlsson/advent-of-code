fn solve_p1(input: &str) -> u32 {
    let length = input.lines().next().unwrap().len();
    let mut bit_counts = vec![0; length];
    let mut lines = 0;
    for line in input.lines() {
        for (i, ch) in line.chars().enumerate() {
            if ch == '1' {
                bit_counts[i] += 1;
            }
        }
        lines += 1;
    }
    let mut gamma = 0;
    for bit_count in bit_counts {
        gamma <<= 1;
        if bit_count > lines / 2 {
            gamma |= 1;
        }
    }
    let epsilon = ((1 << length) - 1) & !gamma;
    epsilon * gamma
}

fn ratio_for_column(col: usize, bits: &[Vec<char>]) -> (usize, usize) {
    let zeros = bits.iter().map(|r| r[col]).filter(|&ch| ch == '0').count();
    (zeros, bits.len() - zeros)
}

fn bits_to_number(bits: &[char]) -> u32 {
    let mut num = 0;
    for &bit in bits {
        num <<= 1;
        num |= (bit == '1') as u32;
    }
    num
}

fn solve_p2(input: &str) -> u32 {
    let bits_orig = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut bits: Vec<_> = bits_orig.clone();

    // Oxygen rating
    for pos in 0..bits[0].len() {
        let (zeros, ones) = ratio_for_column(pos, &bits);
        let bit = if zeros > ones { '0' } else { '1' };
        let new_bits = bits
            .iter()
            .filter(|r| r[pos] == bit)
            .cloned()
            .collect::<Vec<Vec<char>>>();
        bits = new_bits;
        if bits.len() == 1 {
            break;
        }
    }
    let oxygen = bits_to_number(&bits[0]);

    // CO2 scrub
    bits = bits_orig.clone();
    for pos in 0..bits[0].len() {
        let (zeros, ones) = ratio_for_column(pos, &bits);
        let bit = if zeros <= ones { '0' } else { '1' };
        let new_bits = bits
            .iter()
            .filter(|r| r[pos] == bit)
            .cloned()
            .collect::<Vec<Vec<char>>>();
        bits = new_bits;
        if bits.len() == 1 {
            break;
        }
    }
    let co2 = bits_to_number(&bits[0]);
    oxygen * co2
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve_p1(input));
    println!("{}", solve_p2(input));
}
