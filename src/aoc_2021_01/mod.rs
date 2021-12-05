extern crate itertools;

use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.trim().parse::<i64>().unwrap())
        .tuple_windows::<(_, _)>()
        .filter(|(a, b)| a < b)
        .count()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.trim().parse::<i64>().unwrap())
        .tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows::<(_, _)>()
        .filter(|(a, b)| a < b)
        .count()
}

pub static EXAMPLE: &str = std::include_str!("example.txt");
pub static INPUT: &str = std::include_str!("input.txt");

#[cfg(test)]
mod part_1 {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part_1(EXAMPLE), 7);
    }

    #[test]
    fn answer() {
        assert_eq!(part_1(INPUT), 1624);
    }
}

#[cfg(test)]
mod part_2 {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part_2(EXAMPLE), 5);
    }

    #[test]
    fn answer() {
        assert_eq!(part_2(INPUT), 1653);
    }
}
