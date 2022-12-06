use itertools::Itertools;
use regex::Regex;
use std::fs;

pub fn read_crates(input: &str) -> Vec<Vec<char>> {
    let nr_stacks = Regex::new(r"( [1-9]  )+ ([1-9])")
        .unwrap()
        .captures(&input)
        .unwrap()[2]
        .parse::<u32>()
        .unwrap();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    let lines = input.lines().collect_vec();
    for _ in 1..=nr_stacks {
        stacks.push(Vec::new());
    }
    for stack_index in 0..nr_stacks {
        let stack = &mut stacks[stack_index as usize];
        let mut line_nr = 0;
        loop {
            // Check if we haven't reached the bottom.
            let line = lines[line_nr].chars().collect_vec();
            let potential_stack_no = line.get(1).unwrap();
            if *potential_stack_no >= '1' && *potential_stack_no <= '9' {
                break;
            }

            // If not, let's add the crate.
            let symbol = line[(stack_index * 4 + 1) as usize];
            match symbol {
                ' ' => {}
                other => stack.push(other),
            }
            line_nr += 1;
        }
    }
    for i in 0..nr_stacks {
        // Reverse them, so that it becomes a stack from which we can pop.
        stacks[i as usize].reverse();
    }

    stacks
}

struct MoveInstruction {
    amount: usize,
    from: usize,
    to: usize,
}
enum MoveStrategy {
    CrateMover9000,
    CrateMover9001,
}

fn read_instructions(input: &str) -> Vec<MoveInstruction> {
    let regex = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    let captures = regex.captures_iter(&input);

    captures
        .map(|c| MoveInstruction {
            amount: c[1].parse::<usize>().unwrap(),
            from: c[2].parse::<usize>().unwrap(),
            to: c[3].parse::<usize>().unwrap(),
        })
        .collect_vec()
}

fn execute_instructions(
    stacks: &mut Vec<Vec<char>>,
    instructions: &Vec<MoveInstruction>,
    stragety: MoveStrategy,
) -> String {
    for instruction in instructions {
        match stragety {
            MoveStrategy::CrateMover9000 => {
                for _ in 0..instruction.amount {
                    let cr = stacks[instruction.from - 1].pop().unwrap();
                    stacks[instruction.to - 1].push(cr);
                }
            }
            MoveStrategy::CrateMover9001 => {
                let mut to_move: Vec<char> = Vec::new();
                for _ in 0..instruction.amount {
                    let cr = stacks[instruction.from - 1].pop().unwrap();
                    to_move.push(cr);
                }
                for _ in 0..instruction.amount {
                    let cr = to_move.pop().unwrap();
                    stacks[instruction.to - 1].push(cr);
                }
            }
        }
    }
    // Read crates at the topx
    let mut answer = String::new();
    for stack in stacks {
        let top_crate = stack.pop().unwrap().to_string();
        answer += &top_crate;
    }
    answer
}

fn play(input: &str, stragety: MoveStrategy) -> String {
    let mut stacks = read_crates(&input);
    let instructions = read_instructions(&input);
    execute_instructions(&mut stacks, &instructions, stragety)
}

pub fn run() {
    let input = fs::read_to_string("./input/input2022_5.txt").expect("Input file not found");

    println!("Advent of Code 2022 - Day 5");

    println!("Part 1: {}", play(&input, MoveStrategy::CrateMover9000));

    println!("Part 2: {}", play(&input, MoveStrategy::CrateMover9001));
}

#[test]
fn it_recognizes_crate_mover_9000() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    assert_eq!("CMZ", play(&input, MoveStrategy::CrateMover9000))
}

#[test]
fn it_recognizes_crate_mover_9001() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    assert_eq!("MCD", play(&input, MoveStrategy::CrateMover9001))
}
