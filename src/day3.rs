use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashSet;

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day3, part1)]
fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|backpack| {
            let comp_size = backpack.len() / 2;
            let (p1, p2) = backpack.split_at(comp_size);
            let s1: HashSet<u8> = HashSet::from_iter(p1.bytes());
            let s2: HashSet<u8> = HashSet::from_iter(p2.bytes());

            let diff: Vec<_> = s1.intersection(&s2).collect();
            let item = diff.into_iter().next().unwrap();
            let value: u8 = match item {
                b'a'..=b'z' => item - b'a' + 1,
                b'A'..=b'Z' => item - b'A' + 27,
                _ => panic!("Unknown item '{item}'"),
            };
            value as u32
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &[String]) -> u32 {
    input
        .chunks_exact(3)
        .map(|backpacks| {
            let item = backpacks
                .into_iter()
                .map(|backpack| HashSet::from_iter(backpack.bytes()))
                .reduce(|intersection: HashSet<u8>, backpack| {
                    intersection
                        .intersection(&backpack)
                        .into_iter()
                        .map(|e| *e)
                        .collect()
                })
                .unwrap()
                .into_iter()
                .next()
                .unwrap();
            match item {
                b'a'..=b'z' => (item - b'a' + 1) as u32,
                b'A'..=b'Z' => (item - b'A' + 27) as u32,
                _ => panic!("Unknown item '{item}'"),
            }
        })
        .sum()
}
