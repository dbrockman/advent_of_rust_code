extern crate itertools;

use itertools::Itertools;

pub fn part_1(input: &str) -> i64 {
    let numbers = input
        .lines()
        .filter_map(|l| l.trim().parse::<i64>().ok())
        .collect::<Vec<_>>();

    let pair = numbers
        .iter()
        .tuple_combinations()
        .find(|&(a, b)| a + b == 2020)
        .unwrap();

    pair.0 * pair.1
}

pub fn part_2(input: &str) -> i64 {
    let numbers = input
        .lines()
        .filter_map(|l| l.trim().parse::<i64>().ok())
        .collect::<Vec<_>>();

    let tuple = numbers
        .iter()
        .tuple_combinations()
        .find(|&(a, b, c)| a + b + c == 2020)
        .unwrap();

    tuple.0 * tuple.1 * tuple.2
}

pub static EXAMPLE: &str = std::include_str!("example.txt");
pub static INPUT: &str = std::include_str!("input.txt");

#[cfg(test)]
mod part_1 {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part_1(EXAMPLE), 514579);
    }

    #[test]
    fn answer() {
        assert_eq!(part_1(INPUT), 157059);
    }
}

#[cfg(test)]
mod part_2 {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part_2(EXAMPLE), 241861950);
    }

    #[test]
    fn answer() {
        assert_eq!(part_2(INPUT), 165080960);
    }
}
