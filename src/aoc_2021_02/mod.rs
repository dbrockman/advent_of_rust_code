extern crate itertools;

use serde::Deserialize;
use serde_scan;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

pub fn part_1(input: &str) -> i32 {
    let position = input
        .lines()
        .map(|l| serde_scan::from_str::<Direction>(l).unwrap())
        .fold(
            Position {
                horizontal: 0,
                depth: 0,
                aim: 0,
            },
            |pos, dir| match dir {
                Direction::Forward(n) => Position {
                    horizontal: pos.horizontal + n,
                    depth: pos.depth,
                    aim: 0,
                },
                Direction::Down(n) => Position {
                    horizontal: pos.horizontal,
                    depth: pos.depth + n,
                    aim: 0,
                },
                Direction::Up(n) => Position {
                    horizontal: pos.horizontal,
                    depth: pos.depth - n,
                    aim: 0,
                },
            },
        );
    position.horizontal * position.depth
}

pub fn part_2(input: &str) -> i32 {
    let position = input
        .lines()
        .map(|l| serde_scan::from_str::<Direction>(l).unwrap())
        .fold(
            Position {
                horizontal: 0,
                depth: 0,
                aim: 0,
            },
            |pos, dir| match dir {
                Direction::Forward(n) => Position {
                    horizontal: pos.horizontal + n,
                    depth: pos.depth + (n * pos.aim),
                    aim: pos.aim,
                },
                Direction::Down(n) => Position {
                    horizontal: pos.horizontal,
                    depth: pos.depth,
                    aim: pos.aim + n,
                },
                Direction::Up(n) => Position {
                    horizontal: pos.horizontal,
                    depth: pos.depth,
                    aim: pos.aim - n,
                },
            },
        );
    position.horizontal * position.depth
}

pub static EXAMPLE: &str = std::include_str!("example.txt");
pub static INPUT: &str = std::include_str!("input.txt");

#[cfg(test)]
mod part_1 {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part_1(EXAMPLE), 150);
    }

    #[test]
    fn answer() {
        assert_eq!(part_1(INPUT), 1815044);
    }
}

#[cfg(test)]
mod part_2 {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part_2(EXAMPLE), 900);
    }

    #[test]
    fn answer() {
        assert_eq!(part_2(INPUT), 1739283308);
    }
}
