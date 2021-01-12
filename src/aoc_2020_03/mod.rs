extern crate itertools;

use itertools::iterate;

// https://adventofcode.com/2020/day/3

fn count_trees(input: &str, right: usize, down: usize) -> usize {
    let rows: Vec<&str> = input.trim().lines().collect();
    let width = rows.first().unwrap().len();
    let height = rows.len();
    iterate((0, 0), |(x, y)| ((x + right) % width, y + down))
        .take_while(|&(_, y)| y < height)
        .filter(|&(x, y)| rows[y].chars().nth(x).unwrap() == '#')
        .count()
}

pub fn part_1(input: &str) -> usize {
    count_trees(input, 3, 1)
}

pub fn part_2(input: &str) -> usize {
    count_trees(input, 1, 1)
        * count_trees(input, 3, 1)
        * count_trees(input, 5, 1)
        * count_trees(input, 7, 1)
        * count_trees(input, 1, 2)
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
        assert_eq!(part_1(INPUT), 286);
    }
}

#[cfg(test)]
mod part_2 {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part_2(EXAMPLE), 336);
    }

    #[test]
    fn answer() {
        assert_eq!(part_2(INPUT), 3638606400);
    }
}
