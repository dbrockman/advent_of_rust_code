use serde::Deserialize;
use serde_scan::scan;

// https://adventofcode.com/2020/day/2

#[derive(Debug, Deserialize)]
struct PasswordRecord {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

pub fn part_1(input: &str) -> usize {
    let recs = input.trim().lines().map(|l| {
        let rec: PasswordRecord = scan!("{}-{} {}: {}" <- l).unwrap();
        rec
    });

    recs.filter(|r| {
        let count = r.password.matches(r.letter).count();
        r.min <= count && count <= r.max
    })
    .count()
}

pub fn part_2(input: &str) -> usize {
    let recs = input.trim().lines().map(|l| {
        let rec: PasswordRecord = scan!("{}-{} {}: {}" <- l).unwrap();
        rec
    });

    recs.filter(|r| {
        let a = r.password.chars().nth(r.min - 1).unwrap();
        let b = r.password.chars().nth(r.max - 1).unwrap();
        (a == r.letter) ^ (b == r.letter)
    })
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
        assert_eq!(part_1(INPUT), 600);
    }
}

#[cfg(test)]
mod part_2 {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part_2(EXAMPLE), 1);
    }

    #[test]
    fn answer() {
        assert_eq!(part_2(INPUT), 245);
    }
}
