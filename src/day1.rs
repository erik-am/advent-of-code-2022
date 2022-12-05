use std::fs;

fn get_sorted_calories() -> Vec<u32> {
    let input = fs::read_to_string("./input/input2022_1.txt").expect("Input file not found");

    let mut calories = input
        .split("\n\n")
        .map(|calories| {
            calories
                .split('\n')
                .map(|x| x.parse::<u32>().unwrap())
                .fold(0, |acc, curr| acc + curr)
        })
        .collect::<Vec<u32>>();

    calories.sort();
    calories.reverse();

    calories
}

fn get_top_sum(calories: &Vec<u32>, top: usize) -> u32 {
    let total = &calories[..top].iter().fold(0, |acc, curr| acc + curr);

    *total
}

pub fn run() {
    let calories = get_sorted_calories();

    println!("Advent of Code 2022 - Day 1");

    println!("Part 1: {}", get_top_sum(&calories, 1));
    println!("Part 2: {}", get_top_sum(&calories, 3));
}
