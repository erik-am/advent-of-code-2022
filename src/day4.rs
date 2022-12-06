use regex::Regex;
use std::{fs, ops::RangeInclusive};

fn play_part1(lines: &str) -> usize {
    lines.lines().filter(|l| is_fully_overlapping(l)).count()
}

fn parse_ranges(line: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let regex = Regex::new(r"([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)").unwrap();
    let captures = regex.captures(line).unwrap();
    let range1 = captures[1].parse::<u32>().unwrap();
    let range2 = captures[2].parse::<u32>().unwrap();
    let range3 = captures[3].parse::<u32>().unwrap();
    let range4 = captures[4].parse::<u32>().unwrap();

    ((range1..=range2), (range3..=range4))
}

fn is_fully_overlapping(line: &str) -> bool {
    let pair = parse_ranges(line);
    is_fully_included(&pair.0, &pair.1) || is_fully_included(&pair.1, &pair.0)
}

fn is_fully_included(lhs: &RangeInclusive<u32>, rhs: &RangeInclusive<u32>) -> bool {
    let mut lhs = lhs.clone();
    let mut rhs = rhs.clone();
    lhs.all(|x| rhs.any(|y| x == y))
}

fn play_part2(lines: &str) -> usize {
    lines
        .lines()
        .filter(|l| is_partially_overlapping(l))
        .count()
}

fn is_partially_overlapping(line: &str) -> bool {
    let pair = parse_ranges(line);
    is_partially_included(&pair.0, &pair.1) || is_partially_included(&pair.1, &pair.0)
}

fn is_partially_included(lhs: &RangeInclusive<u32>, rhs: &RangeInclusive<u32>) -> bool {
    let mut lhs = lhs.clone();
    let mut rhs = rhs.clone();
    lhs.any(|x| rhs.any(|y| x == y))
}

pub fn run() {
    let input = fs::read_to_string("./input/input2022_4.txt").expect("Input file not found");

    println!("Advent of Code 2022 - Day 4");

    println!("Part 1: {}", play_part1(&input));
    println!("Part 2: {}", play_part2(&input));
}

#[test]
fn it_finds_assignment_pairs_where_one_range_fully_contains_the_other() {
    assert_eq!(
        2,
        play_part1(
            "2-4,6-8\n\
  2-3,4-5\n\
  5-7,7-9\n\
  2-8,3-7\n\
  6-6,4-6\n\
  2-6,4-8"
        )
    )
}

#[test]
fn it_finds_assignment_pairs_where_one_range_partially_contains_the_other() {
    assert_eq!(
        4,
        play_part2(
            "2-4,6-8\n\
2-3,4-5\n\
5-7,7-9\n\
2-8,3-7\n\
6-6,4-6\n\
2-6,4-8"
        )
    )
}
