#[macro_use]
extern crate lazy_static;
use regex::Regex;

type Passport = std::collections::HashMap<String, String>;

fn check_passport_v1(passport: &Passport) -> bool {
    for &key in &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
        if !passport.contains_key(key) {
            return false;
        }
    }
    true
}

fn check_passport_v2(passport: &Passport) -> bool {
    if let Some(byr) = passport.get("byr") {
        let year = byr.parse::<u32>().unwrap();
        if year < 1920 || year > 2002 {
            return false;
        }
    } else {
        return false;
    }
    if let Some(iyr) = passport.get("iyr") {
        let year = iyr.parse::<u32>().unwrap();
        if year < 2010 || year > 2020 {
            return false;
        }
    } else {
        return false;
    }
    if let Some(eyr) = passport.get("eyr") {
        let year = eyr.parse::<u32>().unwrap();
        if year < 2020 || year > 2030 {
            return false;
        }
    } else {
        return false;
    }
    if let Some(hgt) = passport.get("hgt") {
        lazy_static! {
            static ref IN_CHECK: Regex = Regex::new(r"^(\d+)in$").unwrap();
            static ref CM_CHECK: Regex = Regex::new(r"^(\d+)cm$").unwrap();
        }
        if let Some(length) = IN_CHECK.captures(hgt) {
            let len = length[1].parse::<u32>().unwrap();
            if len < 59 || len > 76 {
                return false;
            }
        } else if let Some(length) = CM_CHECK.captures(hgt) {
            let len = length[1].parse::<u32>().unwrap();
            if len < 150 || len > 193 {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }
    if let Some(hcl) = passport.get("hcl") {
        lazy_static! {
            static ref HCL_CHECK: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        }
        if !HCL_CHECK.is_match(hcl) {
            return false;
        }
    } else {
        return false;
    }
    if let Some(ecl) = passport.get("ecl") {
        lazy_static! {
            static ref ECL_CHECK: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
        }
        if !ECL_CHECK.is_match(ecl) {
            return false;
        }
    } else {
        return false;
    }
    if let Some(pid) = passport.get("pid") {
        lazy_static! {
            static ref PID_CHECK: Regex = Regex::new(r"^\d{9}$").unwrap();
        }
        if !PID_CHECK.is_match(pid) {
            return false;
        }
    } else {
        return false;
    }

    true
}

fn solve(input: &str, checker: &dyn Fn(&Passport) -> bool) -> u32 {
    let mut passport = Passport::new();
    let mut valid = 0;
    for line in input.lines() {
        if line == "" {
            valid += checker(&passport) as u32;
            passport.clear();
        } else {
            for key_value in line.split_whitespace() {
                let kv = key_value.split(':').collect::<Vec<_>>();
                passport.insert(kv[0].into(), kv[1].into());
            }
        }
    }
    valid += checker(&passport) as u32;
    valid
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input, &check_passport_v1));
    println!("{}", solve(input, &check_passport_v2));
}

#[test]
fn test_simple() {
    let input = include_str!("input-test");
    assert_eq!(solve(input, &check_passport_v1), 2);

    let input = include_str!("input-invalid-p2");
    assert_eq!(solve(input, &check_passport_v2), 0);

    let input = include_str!("input-valid-p2");
    assert_eq!(solve(input, &check_passport_v2), 4);
}
