use itertools::Itertools;
use std::fs;

fn calculate(input: &str, marker_length: usize) -> Option<usize> {
    for i in (marker_length - 1)..input.chars().count() {
        let uniq = input
            .chars()
            .skip(i - (marker_length - 1))
            .take(marker_length)
            .sorted()
            .dedup()
            .count();
        if uniq == marker_length {
            return Some(i + 1);
        }
    }
    return None;
}

pub fn run() {
    let input = fs::read_to_string("./input/input2022_6.txt").expect("Input file not found");

    println!("Advent of Code 2022 - Day 6");

    println!("Part 1: {}", calculate(&input, 4).unwrap());
    println!("Part 2: {}", calculate(&input, 14).unwrap());
}

#[test]
fn t1() {
    assert_eq!(7, calculate("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4).unwrap())
}

#[test]
fn t2() {
    assert_eq!(5, calculate("bvwbjplbgvbhsrlpgdmjqwftvncz", 4).unwrap())
}

#[test]
fn t3() {
    assert_eq!(6, calculate("nppdvjthqldpwncqszvftbrmjlhg", 4).unwrap())
}

#[test]
fn t4() {
    assert_eq!(
        26,
        calculate("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14).unwrap()
    )
}
