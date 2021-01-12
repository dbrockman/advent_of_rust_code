extern crate itertools;

use itertools::Itertools;

// https://adventofcode.com/2020/day/6

fn count_group_any(group: &str) -> usize {
    group
        .chars()
        .filter(|c| c.is_ascii_lowercase())
        .unique()
        .count()
}

fn count_group_all(group: &str) -> usize {
    let people = group.lines().count();
    group
        .chars()
        .filter(char::is_ascii_lowercase)
        .counts()
        .values()
        .filter(|count| count == &&people)
        .count()
}

pub fn part_1(input: &str) -> usize {
    input.trim().split("\n\n").map(|s| count_group_any(s)).sum()
}

pub fn part_2(input: &str) -> usize {
    input.trim().split("\n\n").map(|s| count_group_all(s)).sum()
}

pub static EXAMPLE: &str = std::include_str!("example.txt");
pub static INPUT: &str = std::include_str!("input.txt");

#[cfg(test)]
mod part_1 {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part_1(EXAMPLE), 11);
    }

    #[test]
    fn answer() {
        assert_eq!(part_1(INPUT), 6259);
    }
}

#[cfg(test)]
mod part_2 {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part_2(EXAMPLE), 6);
    }

    #[test]
    fn answer() {
        assert_eq!(part_2(INPUT), 3178);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_group_any() {
        assert_eq!(count_group_any(""), 0);
        assert_eq!(count_group_any("abc"), 3);
        assert_eq!(count_group_any("a\nb\nc"), 3);
        assert_eq!(count_group_any("ab\nac"), 3);
        assert_eq!(count_group_any("a\na\na\na"), 1);
        assert_eq!(count_group_any("b"), 1);
    }

    #[test]
    fn test_count_group_all() {
        assert_eq!(count_group_all(""), 0);
        assert_eq!(count_group_all("abc"), 3);
        assert_eq!(count_group_all("a\nb\nc"), 0);
        assert_eq!(count_group_all("ab\nac"), 1);
        assert_eq!(count_group_all("a\na\na\na"), 1);
        assert_eq!(count_group_all("b"), 1);
    }
}
