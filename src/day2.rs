use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Move {
    fn from(input: char) -> Self {
        match input {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            _ => panic!("Unknown Move"),
        }
    }
}

impl Move {
    fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Clone, Copy)]
enum RoundResult {
    Loss,
    Draw,
    Win,
}

impl RoundResult {
    fn score(&self) -> u32 {
        match self {
            RoundResult::Loss => 0,
            RoundResult::Draw => 3,
            RoundResult::Win => 6,
        }
    }
}

impl From<char> for RoundResult {
    fn from(input: char) -> Self {
        match input {
            'X' => RoundResult::Loss,
            'Y' => RoundResult::Draw,
            'Z' => RoundResult::Win,
            _ => panic!("Unknown result"),
        }
    }
}

#[derive(Clone, Copy)]
struct Round {
    mine: Move,
    result: RoundResult,
}

impl Round {
    fn score(&self) -> u32 {
        self.mine.score() + self.result.score()
    }

    fn from_moves(their: Move, mine: Move) -> Self {
        let result = match (their, mine) {
            (Move::Rock, Move::Rock) => RoundResult::Draw,
            (Move::Paper, Move::Paper) => RoundResult::Draw,
            (Move::Scissors, Move::Scissors) => RoundResult::Draw,

            (Move::Rock, Move::Scissors) => RoundResult::Loss,
            (Move::Paper, Move::Rock) => RoundResult::Loss,
            (Move::Scissors, Move::Paper) => RoundResult::Loss,

            (Move::Rock, Move::Paper) => RoundResult::Win,
            (Move::Paper, Move::Scissors) => RoundResult::Win,
            (Move::Scissors, Move::Rock) => RoundResult::Win,
        };

        Self { mine, result }
    }

    fn from_result(their: Move, result: RoundResult) -> Self {
        let mine = match (their, result) {
            (Move::Rock, RoundResult::Draw) => Move::Rock,
            (Move::Paper, RoundResult::Draw) => Move::Paper,
            (Move::Scissors, RoundResult::Draw) => Move::Scissors,

            (Move::Rock, RoundResult::Loss) => Move::Scissors,
            (Move::Paper, RoundResult::Loss) => Move::Rock,
            (Move::Scissors, RoundResult::Loss) => Move::Paper,

            (Move::Rock, RoundResult::Win) => Move::Paper,
            (Move::Paper, RoundResult::Win) => Move::Scissors,
            (Move::Scissors, RoundResult::Win) => Move::Rock,
        };

        Self { mine, result }
    }
}

#[aoc_generator(day2, part1)]
fn parse_input_part1(input: &str) -> Vec<Round> {
    let mut moves = Vec::new();

    for line in input.lines() {
        let mut chars = line.chars();
        let their = chars.next().unwrap().into();
        let mine = chars.last().unwrap().into();
        moves.push(Round::from_moves(their, mine))
    }

    moves
}

#[aoc_generator(day2, part2)]
fn parse_input_part2(input: &str) -> Vec<Round> {
    let mut moves = Vec::new();

    for line in input.lines() {
        let mut chars = line.chars();
        let their = chars.next().unwrap().into();
        let result = chars.last().unwrap().into();
        moves.push(Round::from_result(their, result))
    }

    moves
}

#[aoc(day2, part1)]
fn part1(rounds: &[Round]) -> u32 {
    rounds.iter().map(|round| round.score()).sum()
}

#[aoc(day2, part2)]
fn part2(rounds: &[Round]) -> u32 {
    rounds.iter().map(|round| round.score()).sum()
}
