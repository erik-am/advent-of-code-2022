use array_tool::vec::Intersect;
use itertools::Itertools;
use std::fs;

fn find_item_to_rearrange(rucksack: &str) -> u32 {
    let items = rucksack.chars().collect::<Vec<char>>();
    let length = items.len() / 2;

    items[..length]
        .to_vec()
        .intersect(items[length..].to_vec())
        .iter()
        .map(|item| get_priority(*item))
        .sum::<u32>()
}

fn get_priority(item: char) -> u32 {
    if item.is_lowercase() {
        item as u32 - 96
    } else {
        item as u32 - 38
    }
}

fn find_common_item(rucksacks: &[&str]) -> char {
    *(rucksacks
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .reduce(|acc, curr| acc.intersect(curr))
        .expect("No rucksacks")
        .first()
        .expect("Expected one"))
}

pub fn run() {
    let input = fs::read_to_string("./input/input2022_3.txt").expect("Input file not found");

    println!("Advent of Code 2022 - Day 3");

    println!(
        "Part 1: {}",
        input
            .lines()
            .map(|rucksack| find_item_to_rearrange(rucksack))
            .sum::<u32>()
    );

    let lines = input.lines().collect::<Vec<&str>>();
    let grouped_rucksacks = lines.chunks(3).collect_vec();

    let part2 = grouped_rucksacks
        .iter()
        .map(|group| find_common_item(group))
        .map(|item| get_priority(item))
        .sum::<u32>();

    println!("Part 2: {}", part2);
}
