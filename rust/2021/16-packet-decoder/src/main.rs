use itertools::Itertools;

fn parse_slice(binary: &[u8]) -> u64 {
    binary
        .iter()
        .fold(0u64, |acc, bit| (acc << 1) | *bit as u64)
}

/// All parser functions returns (bits_consumed, version_accumulate_sum, value)

fn parse_literal(binary: &[u8]) -> (usize, u64, u64) {
    let version = parse_slice(&binary[0..3]);
    let mut pos = 6;
    let mut value = 0;
    loop {
        let group = parse_slice(&binary[(pos + 1)..(pos + 5)]);
        value = (value << 4) | group;
        if binary[pos] == 0 {
            break;
        }
        pos += 5;
    }
    (pos + 5, version, value)
}

fn parse_operator(binary: &[u8]) -> (usize, u64, Vec<u64>) {
    let version = parse_slice(&binary[0..3]);
    let mut values = Vec::new();
    if binary[6] == 0 {
        // 15 bits
        let length = parse_slice(&binary[7..22]) as usize;
        let mut pos = 22;
        let mut consumed_total = 0;
        let mut version_sum = version;
        while consumed_total < length {
            let (consumed, sum, value) = parse_packet(&binary[pos..]);
            consumed_total += consumed;
            pos += consumed;
            version_sum += sum;
            values.push(value);
        }
        assert_eq!(consumed_total, length);
        (pos, version_sum, values)
    } else {
        // 11 bits
        let packets = parse_slice(&binary[7..18]) as usize;
        let mut pos = 18;
        let mut version_sum = version;
        for _ in 0..packets {
            let (consumed, sum, value) = parse_packet(&binary[pos..]);
            pos += consumed;
            version_sum += sum;
            values.push(value);
        }
        (pos, version_sum, values)
    }
}

fn parse_packet(binary: &[u8]) -> (usize, u64, u64) {
    let packet_type = parse_slice(&binary[3..6]);
    if packet_type == 4 {
        parse_literal(binary)
    } else {
        let (nbits, version_sum, values) = parse_operator(binary);
        match packet_type {
            0 => (nbits, version_sum, values.iter().sum()),
            1 => (nbits, version_sum, values.iter().product()),
            2 => (nbits, version_sum, *values.iter().min().unwrap()),
            3 => (nbits, version_sum, *values.iter().max().unwrap()),
            5 => (nbits, version_sum, (values[0] > values[1]) as u64),
            6 => (nbits, version_sum, (values[0] < values[1]) as u64),
            7 => (nbits, version_sum, (values[0] == values[1]) as u64),
            _ => panic!("Unkown operator: {:?}", packet_type),
        }
    }
}

fn parse(input: &str) -> Vec<u8> {
    let binary_str = input
        .trim()
        .chars()
        .map(|ch| format!("{:04b}", ch.to_digit(16).unwrap()))
        .join("");
    binary_str
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>()
}

fn solve(input: &str) -> (u64, u64) {
    let binary = parse(input);
    let (_, version_sum, result) = parse_packet(&binary);
    (version_sum, result)
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input));
}

#[test]
fn test_literal() {
    let (bits, version, value) = parse_packet(&parse("D2FE28"));
    assert_eq!(version, 6);
    assert_eq!(bits, 21);
    assert_eq!(value, 2021);
}

#[test]
fn test_operator_1() {
    let (bits, version, value) = parse_packet(&parse("38006F45291200"));
    assert_eq!(version, 9);
    assert_eq!(bits, 3 + 3 + 1 + 15 + 11 + 16);
    assert_eq!(value, 1);
}

#[test]
fn test_operator_2() {
    let (bits, version, value) = parse_packet(&parse("EE00D40C823060"));
    assert_eq!(version, 7 + 2 + 4 + 1);
    assert_eq!(bits, 3 + 3 + 1 + 11 + 11 + 11 + 11);
    assert_eq!(value, 3);
}

#[test]
fn test_bunch_1() {
    assert_eq!(solve("8A004A801A8002F478").0, 16);
}

#[test]
fn test_bunch_2() {
    assert_eq!(solve("620080001611562C8802118E34").0, 12);
}

#[test]
fn test_bunch_3() {
    assert_eq!(solve("C0015000016115A2E0802F182340").0, 23);
}

#[test]
fn test_bunch_4() {
    assert_eq!(solve("A0016C880162017C3686B18A3D4780").0, 31);
}
