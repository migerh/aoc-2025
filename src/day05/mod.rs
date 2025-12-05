use std::{cmp::max, str::FromStr};

use anyhow::{Context, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::AocError;

#[derive(Debug, Clone)]
pub struct Range {
    start: u128,
    end: u128,
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spl = s.split('-');
        let start = spl
            .next()
            .ok_or(AocError::GenericError)
            .context("Could not find start")?
            .parse::<u128>()?;
        let end = spl
            .next()
            .ok_or(AocError::GenericError)
            .context("Could not find end")?
            .parse::<u128>()?;

        Ok(Range { start, end })
    }
}

impl Range {
    fn is_within(&self, value: u128) -> bool {
        value >= self.start && value <= self.end
    }
}

#[aoc_generator(day05)]
pub fn input_generator(input: &str) -> Result<(Vec<Range>, Vec<u128>)> {
    let mut split = input.split("\n\n");
    let first = split.next().ok_or(AocError::GenericError)?;
    let second = split.next().ok_or(AocError::GenericError)?;

    let ranges = first
        .lines()
        .map(Range::from_str)
        .collect::<Result<Vec<_>>>()?;
    let ingredients = second
        .lines()
        .map(|l| Ok(l.parse::<u128>()?))
        .collect::<Result<Vec<_>>>()?;

    Ok((ranges, ingredients))
}

fn is_in_any(ranges: &[Range], value: u128) -> bool {
    ranges.iter().any(|r| r.is_within(value))
}

#[aoc(day05, part1)]
pub fn solve_part1(input: &(Vec<Range>, Vec<u128>)) -> Result<usize> {
    let (ranges, ingredients) = input;

    Ok(ingredients
        .iter()
        .filter(|v| is_in_any(ranges, **v))
        .count())
}

fn merge(ranges: &[Range]) -> Vec<Range> {
    let mut ranges = ranges.to_vec();
    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut current = ranges[0].clone();
    let mut merged = vec![];

    ranges.iter().skip(1).for_each(|range| {
        if current.end >= range.start {
            current.end = max(range.end, current.end);
        } else {
            merged.push(current.clone());
            current = range.clone();
        }
    });
    merged.push(current);

    merged
}

#[aoc(day05, part2)]
pub fn solve_part2(input: &(Vec<Range>, Vec<u128>)) -> Result<u128> {
    let (ranges, _) = input;

    let merged = merge(ranges);
    Ok(merged.iter().map(|r| r.end - r.start + 1).sum::<u128>())
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Result<(Vec<Range>, Vec<u128>)> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        input_generator(input)
    }

    #[test]
    fn part1() -> Result<()> {
        let input = input()?;
        Ok(assert_eq!(solve_part1(&input)?, 3))
    }

    #[test]
    fn part2() -> Result<()> {
        let input = input()?;
        Ok(assert_eq!(solve_part2(&input)?, 14))
    }
}
