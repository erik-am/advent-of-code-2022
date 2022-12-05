use core::slice::Iter;
use std::fs;

enum PlayingStrategy {
    MyChoice,
    MyOutcome,
}

const SCORE_ROCK: u32 = 1;
const SCORE_PAPER: u32 = 2;
const SCORE_SCISSORS: u32 = 3;

const SCORE_LOST: u32 = 0;
const SCORE_DRAW: u32 = 3;
const SCORE_WON: u32 = 6;

fn play(input_lines: Iter<&str>, strategy: PlayingStrategy) -> u32 {
    input_lines
        .map(|line| play_round(line, &strategy))
        .fold(0, |acc, curr| acc + curr)
}

fn play_round(input_line: &str, strategy: &PlayingStrategy) -> u32 {
    match strategy {
        PlayingStrategy::MyChoice => match input_line {
            "A X" => SCORE_ROCK + SCORE_DRAW,
            "A Y" => SCORE_PAPER + SCORE_WON,
            "A Z" => SCORE_SCISSORS + SCORE_LOST,
            "B X" => SCORE_ROCK + SCORE_LOST,
            "B Y" => SCORE_PAPER + SCORE_DRAW,
            "B Z" => SCORE_SCISSORS + SCORE_WON,
            "C X" => SCORE_ROCK + SCORE_WON,
            "C Y" => SCORE_PAPER + SCORE_LOST,
            "C Z" => SCORE_SCISSORS + SCORE_DRAW,
            other => panic!("Unrecognized input {}", other),
        },
        PlayingStrategy::MyOutcome => match input_line {
            "A X" => SCORE_SCISSORS + SCORE_LOST,
            "A Y" => SCORE_ROCK + SCORE_DRAW,
            "A Z" => SCORE_PAPER + SCORE_WON,
            "B X" => SCORE_ROCK + SCORE_LOST,
            "B Y" => SCORE_PAPER + SCORE_DRAW,
            "B Z" => SCORE_SCISSORS + SCORE_WON,
            "C X" => SCORE_PAPER + SCORE_LOST,
            "C Y" => SCORE_SCISSORS + SCORE_DRAW,
            "C Z" => SCORE_ROCK + SCORE_WON,
            other => panic!("Unrecognized input {}", other),
        },
    }
}

pub fn run() {
    let input = fs::read_to_string("./input/input2022_2.txt").expect("Input file not found");
    let input_lines = input.split('\n').collect::<Vec<&str>>();

    println!("Advent of Code 2022 - Day 2");

    println!(
        "Part 1: {}",
        play(input_lines.iter(), PlayingStrategy::MyChoice)
    );
    println!(
        "Part 2: {}",
        play(input_lines.iter(), PlayingStrategy::MyOutcome)
    );
}
