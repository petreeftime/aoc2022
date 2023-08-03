use aoc_runner_derive::{aoc, aoc_generator};

use std::{str::FromStr};

struct Range {
    lo: u32,
    hi: u32
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       s.split_once('-').map(|(v1, v2)| {
           Range {
               lo: v1.parse().unwrap(),
               hi: v2.parse().unwrap(),
           }
       }).ok_or(())
    }

}


#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<(Range, Range)> {
    input.lines().map(|line|
        line.split_once(',').map(|(r1, r2)| {
            (r1.parse().unwrap(), r2.parse().unwrap())
        }
        ).unwrap()
    ).collect()
}

#[aoc(day4, part1)]
fn part1(input: &[(Range, Range)]) -> usize {
    input.iter().filter(|(r1, r2)| {
        (r1.lo <= r2.lo && r2.hi <= r1.hi) ||
        (r1.lo >= r2.lo && r2.hi >= r1.hi) 
    }).count()
}

#[aoc(day4, part2)]
fn part2(input: &[(Range, Range)]) -> usize {
    input.iter().filter(|(r1, r2)| {
        (r1.lo <= r2.lo && r2.hi <= r1.hi) ||
        (r1.lo >= r2.lo && r2.hi >= r1.hi) ||
        (r1.lo <= r2.lo && r2.lo <= r1.hi) ||
        (r1.lo >= r2.lo && r1.lo <= r2.hi)
    }).count()
}
