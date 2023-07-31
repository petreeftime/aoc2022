use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut elf = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            result.push(elf);
            elf = Vec::new();
        } else {
            let calories = line.parse::<i32>().unwrap();
            elf.push(calories);
        }
    }
    result.push(elf);
    result
}

#[aoc(day1, part1)]
fn part1(input: &Vec<Vec<i32>>) -> i32 {
    input.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
fn part2(input: &Vec<Vec<i32>>) -> i32 {
    let mut sums = input
        .iter()
        .map(|elf| elf.iter().sum())
        .collect::<Vec<i32>>();
    sums.sort();
    sums.pop().unwrap_or(0) + sums.pop().unwrap_or(0) + sums.pop().unwrap_or(0)
}
