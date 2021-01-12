// https://adventofcode.com/2020/day/4

struct Passport<'a> {
    /// Birth Year
    byr: &'a str,

    /// Issue Year
    iyr: &'a str,

    /// Expiration Year
    eyr: &'a str,

    /// Height
    hgt: &'a str,

    /// Hair Color
    hcl: &'a str,

    /// Eye Color
    ecl: &'a str,

    /// Passport ID
    pid: &'a str,
}

fn get_field<'a>(s: &'a str, field: &str) -> Option<&'a str> {
    s.split_whitespace()
        .find(|part| part.starts_with(field))
        .unwrap_or("")
        .strip_prefix(field)
        .unwrap_or("")
        .strip_prefix(":")
}

fn try_create_passport(s: &str) -> Option<Passport> {
    let byr = get_field(s, "byr");
    let iyr = get_field(s, "iyr");
    let eyr = get_field(s, "eyr");
    let hgt = get_field(s, "hgt");
    let hcl = get_field(s, "hcl");
    let ecl = get_field(s, "ecl");
    let pid = get_field(s, "pid");
    match (byr, iyr, eyr, hgt, hcl, ecl, pid) {
        (Some(byr), Some(iyr), Some(eyr), Some(hgt), Some(hcl), Some(ecl), Some(pid)) => {
            Some(Passport {
                byr,
                iyr,
                eyr,
                hgt,
                hcl,
                ecl,
                pid,
            })
        }
        _ => None,
    }
}

fn validate_int_range(s: &str, lo: i32, hi: i32) -> bool {
    match s.parse::<i32>().ok() {
        Some(i) => lo <= i && i <= hi,
        _ => false,
    }
}

fn validate_height(hgt: &str) -> bool {
    // - hgt (Height) - a number followed by either cm or in:
    //   - If cm, the number must be at least 150 and at most 193.
    //   - If in, the number must be at least 59 and at most 76.
    match (hgt.strip_suffix("cm"), hgt.strip_suffix("in")) {
        (Some(cm), None) => validate_int_range(cm, 150, 193),
        (None, Some(inches)) => validate_int_range(inches, 59, 76),
        _ => false,
    }
}

fn validate_hair_color(hcl: &str) -> bool {
    hcl.len() == 7
        && hcl.starts_with('#')
        && hcl.chars().skip(1).all(|c| match c {
            '0'..='9' => true,
            'a'..='f' => true,
            _ => false,
        })
}

fn validate_eye_color(ecl: &str) -> bool {
    ecl == "amb"
        || ecl == "blu"
        || ecl == "brn"
        || ecl == "gry"
        || ecl == "grn"
        || ecl == "hzl"
        || ecl == "oth"
}

fn validate_passport_id(pid: &str) -> bool {
    pid.len() == 9
        && pid.chars().all(|c| match c {
            '0'..='9' => true,
            _ => false,
        })
}

fn validate_passport(passport: &Passport) -> bool {
    // - byr (Birth Year) - four digits; at least 1920 and at most 2002.
    if validate_int_range(passport.byr, 1920, 2002) == false {
        return false;
    }

    // - iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    if validate_int_range(passport.iyr, 2010, 2020) == false {
        return false;
    }

    // - eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    if validate_int_range(passport.eyr, 2020, 2030) == false {
        return false;
    }

    // - hgt (Height) - a number followed by either cm or in:
    //   - If cm, the number must be at least 150 and at most 193.
    //   - If in, the number must be at least 59 and at most 76.
    if validate_height(passport.hgt) == false {
        return false;
    }

    // - hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    if validate_hair_color(passport.hcl) == false {
        return false;
    }

    // - ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    if validate_eye_color(passport.ecl) == false {
        return false;
    }

    // - pid (Passport ID) - a nine-digit number, including leading zeroes.
    if validate_passport_id(passport.pid) == false {
        return false;
    }

    true
}

pub fn part_1(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .filter_map(|l| try_create_passport(l))
        .count()
}

pub fn part_2(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .filter_map(|l| try_create_passport(l))
        .filter(|p| validate_passport(p))
        .count()
}

pub static EXAMPLE: &str = std::include_str!("example.txt");
pub static INPUT: &str = std::include_str!("input.txt");

#[cfg(test)]
mod part_1 {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part_1(EXAMPLE), 2);
    }

    #[test]
    fn answer() {
        assert_eq!(part_1(INPUT), 233);
    }
}

#[cfg(test)]
mod part_2 {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part_2(EXAMPLE), 2);
    }

    #[test]
    fn answer() {
        assert_eq!(part_2(INPUT), 111);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_int_range() {
        assert_eq!(validate_int_range("", 1, 5), false);
        assert_eq!(validate_int_range("garbage", 1, 5), false);
        assert_eq!(validate_int_range("0", 1, 5), false);
        assert_eq!(validate_int_range("1", 1, 5), true);
        assert_eq!(validate_int_range("2", 1, 5), true);
        assert_eq!(validate_int_range("3", 1, 5), true);
        assert_eq!(validate_int_range("4", 1, 5), true);
        assert_eq!(validate_int_range("5", 1, 5), true);
        assert_eq!(validate_int_range("6", 1, 5), false);
    }

    #[test]
    fn test_validate_height() {
        assert_eq!(validate_height(""), false);
        assert_eq!(validate_height("garbage"), false);

        assert_eq!(validate_height("149cm"), false);
        assert_eq!(validate_height("150cm"), true);
        assert_eq!(validate_height("193cm"), true);
        assert_eq!(validate_height("194cm"), false);

        assert_eq!(validate_height("58in"), false);
        assert_eq!(validate_height("59in"), true);
        assert_eq!(validate_height("76in"), true);
        assert_eq!(validate_height("77in"), false);
    }

    #[test]
    fn test_validate_hair_color() {
        assert_eq!(validate_hair_color(""), false);
        assert_eq!(validate_hair_color("garbage"), false);
        assert_eq!(validate_hair_color("#xxxxxx"), false);
        assert_eq!(validate_hair_color("#12345"), false);
        assert_eq!(validate_hair_color("#1234567"), false);
        assert_eq!(validate_hair_color("#123456"), true);
        assert_eq!(validate_hair_color("#abcdef"), true);
    }

    #[test]
    fn test_validate_eye_color() {
        assert_eq!(validate_eye_color(""), false);
        assert_eq!(validate_eye_color("xxx"), false);
        assert_eq!(validate_eye_color("amb"), true);
        assert_eq!(validate_eye_color("blu"), true);
        assert_eq!(validate_eye_color("brn"), true);
        assert_eq!(validate_eye_color("gry"), true);
        assert_eq!(validate_eye_color("grn"), true);
        assert_eq!(validate_eye_color("hzl"), true);
        assert_eq!(validate_eye_color("oth"), true);
    }

    #[test]
    fn test_validate_passport_id() {
        assert_eq!(validate_passport_id(""), false);
        assert_eq!(validate_passport_id("12345678"), false);
        assert_eq!(validate_passport_id("1234567890"), false);
        assert_eq!(validate_passport_id("123456789"), true);
        assert_eq!(validate_passport_id("12345678a"), false);
    }
}
