extern crate itertools;

use itertools::Itertools;

// https://adventofcode.com/2020/day/5

#[derive(Debug, PartialEq, Eq, Clone)]
struct Seat {
    row: u32,
    col: u32,
    id: u32,
}

fn calculate_seat_id(row: u32, col: u32) -> u32 {
    (row * 8) + col
}

fn lower_half(t: (u32, u32)) -> (u32, u32) {
    let lo = t.0 as f64;
    let hi = t.1 as f64;
    let mid = hi - ((hi - lo) / 2_f64);
    (lo as u32, mid.floor() as u32)
}

fn upper_half(t: (u32, u32)) -> (u32, u32) {
    let lo = t.0 as f64;
    let hi = t.1 as f64;
    let mid = lo + ((hi - lo) / 2_f64);
    (mid.ceil() as u32, hi as u32)
}

fn get_row_range(s: &str) -> (u32, u32) {
    s.chars().fold((0, 127), |acc, c| match c {
        'F' => lower_half(acc),
        'B' => upper_half(acc),
        _ => acc,
    })
}

fn get_col_range(s: &str) -> (u32, u32) {
    s.chars().fold((0, 7), |acc, c| match c {
        'L' => lower_half(acc),
        'R' => upper_half(acc),
        _ => acc,
    })
}

fn seat_from_str(s: &str) -> Seat {
    let row = get_row_range(s).0;
    let col = get_col_range(s).0;
    let id = calculate_seat_id(row, col);
    Seat { row, col, id }
}

pub fn part_1(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|s| seat_from_str(s).id)
        .max()
        .unwrap_or(0)
}

pub fn part_2(input: &str) -> u32 {
    let mut vec = input
        .trim()
        .lines()
        .map(|s| seat_from_str(s))
        .collect::<Vec<_>>();

    vec.sort_by_key(|s| s.id);

    for (a, b) in vec.iter().tuple_windows() {
        if (a.id + 2) == b.id {
            return a.id + 1;
        }
    }
    panic!("Didn't find the seat!");
}

pub static EXAMPLE: &str = std::include_str!("example.txt");
pub static INPUT: &str = std::include_str!("input.txt");

#[cfg(test)]
mod part_1 {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part_1(EXAMPLE), 820);
    }

    #[test]
    fn answer() {
        assert_eq!(part_1(INPUT), 989);
    }
}

#[cfg(test)]
mod part_2 {
    use super::*;

    #[test]
    fn answer() {
        assert_eq!(part_2(INPUT), 548);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_seat_id() {
        assert_eq!(calculate_seat_id(44, 5), 357);
        assert_eq!(calculate_seat_id(70, 7), 567);
        assert_eq!(calculate_seat_id(14, 7), 119);
        assert_eq!(calculate_seat_id(102, 4), 820);
    }

    #[test]
    fn test_get_row_range() {
        assert_eq!(get_row_range("F"), (0, 63));
        assert_eq!(get_row_range("FB"), (32, 63));
        assert_eq!(get_row_range("FBF"), (32, 47));
        assert_eq!(get_row_range("FBFB"), (40, 47));
        assert_eq!(get_row_range("FBFBB"), (44, 47));
        assert_eq!(get_row_range("FBFBBF"), (44, 45));
        assert_eq!(get_row_range("FBFBBFF"), (44, 44));
        assert_eq!(get_row_range("FBFBBFFRLR"), (44, 44));
    }

    #[test]
    fn test_get_col_range() {
        assert_eq!(get_col_range("R"), (4, 7));
        assert_eq!(get_col_range("RL"), (4, 5));
        assert_eq!(get_col_range("RLR"), (5, 5));
        assert_eq!(get_col_range("FBFBBFFRLR"), (5, 5));
    }

    #[test]
    fn test_seat_from_str() {
        assert_eq!(
            seat_from_str("BFFFBBFRRR"),
            Seat {
                row: 70,
                col: 7,
                id: 567
            }
        );
        assert_eq!(
            seat_from_str("FFFBBBFRRR"),
            Seat {
                row: 14,
                col: 7,
                id: 119
            }
        );
        assert_eq!(
            seat_from_str("BBFFBBFRLL"),
            Seat {
                row: 102,
                col: 4,
                id: 820
            }
        );
    }
}
