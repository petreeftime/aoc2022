use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Move {
    quantity: usize,
    stack_src: usize,
    stack_dst: usize
}

#[derive(Debug)]
struct Crane {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move> 
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for line in input.lines().rev().skip(1) {
        let num_stacks = line.chars().skip(1).step_by(4).count();
        if result.len() < num_stacks {
            result.resize(num_stacks, Vec::new());
        }
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if !c.is_whitespace() {
                result[i].push(c);
            }
        }
    }
    result
}

fn parse_moves(input: &str) -> Vec<Move> {
    input.lines().map(|line| {
        let line: Vec<&str> = line.split_whitespace().collect();
        Move {
            quantity: line[1].parse().unwrap(),
            stack_src: line[3].parse::<usize>().unwrap() - 1,
            stack_dst: line[5].parse::<usize>().unwrap() - 1,
        }
    }).collect()
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Crane {
    let (stacks_input, moves_input) = input.split_once("\n\n").unwrap();

    let stacks = parse_stacks(stacks_input);
    let moves = parse_moves(moves_input);

    Crane {
        stacks,
        moves
    }
}

#[aoc(day5, part1)]
fn part1(input: &Crane) -> String {
    let mut stacks = input.stacks.clone();
    for crane_move in input.moves.iter() {
        for _ in 0..crane_move.quantity {
            if let Some(c) = stacks[crane_move.stack_src].pop() {
                stacks[crane_move.stack_dst].push(c);
            }
        }
    }
    let mut result = String::new();
    for stack in stacks.iter() {
        if let Some(c) = stack.last() {
            result.push(*c);
        }
    }
    result
}

#[aoc(day5, part2)]
fn part2(input: &Crane) -> String {
    let mut stacks = input.stacks.clone();
    for crane_move in input.moves.iter() {
        let stack_src = &mut stacks[crane_move.stack_src];
        let mut crates = stack_src.split_off(stack_src.len() - crane_move.quantity);

        let stack_dst = &mut stacks[crane_move.stack_dst];
        stack_dst.append(&mut crates);
    }
    let mut result = String::new();
    for stack in stacks.iter() {
        if let Some(c) = stack.last() {
            result.push(*c);
        }
    }
    result
}
