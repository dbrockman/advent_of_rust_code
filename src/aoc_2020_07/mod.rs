extern crate itertools;

use itertools::Itertools;
use std::collections::HashMap;

// https://adventofcode.com/2020/day/7

fn parse_line_spec(spec: &str) -> Vec<(usize, String)> {
    let mut vec: Vec<(usize, String)> = Vec::new();
    for s in spec.split(", ") {
        if s != "no other bags." {
            let mut words = s.split(" ").collect_vec();
            assert!(words.len() > 2);
            if let Some(count) = words.remove(0).parse::<usize>().ok() {
                words.pop();
                vec.push((count, words.join(" ")));
            }
        }
    }
    vec
}

fn parse_line(line: &str) -> (String, Vec<(usize, String)>) {
    let parts = line.trim().split(" bags contain ").collect_vec();
    assert!(parts.len() == 2);
    let name = parts[0].to_string();
    let vec = parse_line_spec(parts[1]);
    (name, vec)
}

fn parse_input(input: &str) -> HashMap<String, Vec<(usize, String)>> {
    let mut hm: HashMap<String, Vec<(usize, String)>> = HashMap::new();
    for line in input.trim().lines() {
        let (name, vec) = parse_line(line);
        hm.insert(name, vec);
    }
    hm
}

pub fn part_1(input: &str) -> usize {
    let hm = parse_input(input);
    hm.len()
}

pub fn part_2(input: &str) -> usize {
    input.trim().lines().count()
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
    fn test_parse_line_spec() {
        assert_eq!(parse_line_spec("no other bags.").len(), 0);
        assert_eq!(
            parse_line_spec("1 shiny gold bag."),
            vec![(1, "shiny gold".to_string())]
        );
        assert_eq!(
            parse_line_spec("3 faded blue bags, 4 dotted black bags."),
            vec![
                (3, "faded blue".to_string()),
                (4, "dotted black".to_string())
            ]
        );
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("faded blue bags contain no other bags."),
            ("faded blue".to_string(), vec![])
        );
        assert_eq!(
            parse_line("bright white bags contain 1 shiny gold bag."),
            (
                "bright white".to_string(),
                vec![(1, "shiny gold".to_string())]
            )
        );
        assert_eq!(
            parse_line("dark olive bags contain 3 faded blue bags, 4 dotted black bags."),
            (
                "dark olive".to_string(),
                vec![
                    (3, "faded blue".to_string()),
                    (4, "dotted black".to_string())
                ]
            )
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input("").len(), 0);
        let map = parse_input(EXAMPLE);
        assert_eq!(map.len(), 9);
        assert_eq!(map.get(&"faded blue".to_string()).unwrap().len(), 0);
        assert_eq!(
            map.get(&"bright white".to_string()).unwrap(),
            &vec![(1_usize, "shiny gold".to_string())]
        );
        assert_eq!(
            map.get(&"dark olive".to_string()).unwrap(),
            &vec![
                (3_usize, "faded blue".to_string()),
                (4_usize, "dotted black".to_string())
            ]
        );
    }
}
