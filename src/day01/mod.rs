use std::str::FromStr;

use anyhow::{Context, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::AocError;

#[derive(Debug)]
pub enum Op {
    Left(i32),
    Right(i32),
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let dir = chars
            .next()
            .ok_or(AocError::GenericError)
            .context("Could not find direction")?;
        let num = chars
            .collect::<String>()
            .parse::<i32>()
            .context("Could not parse amount")?;
        match dir {
            'R' => Ok(Op::Right(num)),
            'L' => Ok(Op::Left(num)),
            _ => Err(AocError::GenericError).context("Unexpected direction")?,
        }
    }
}

#[aoc_generator(day01)]
pub fn input_generator(input: &str) -> Result<Vec<Op>> {

    let result = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(Op::from_str)
        .collect::<Result<Vec<_>>>()
        .unwrap();
    Ok(result)
}

#[aoc(day01, part1)]
pub fn solve_part1(input: &[Op]) -> Result<i32> {
    let mut dial = 50;
    let mut num = 0;

    for op in input {
        let diff = match op {
            Op::Left(v) => -v,
            Op::Right(v) => *v,
        };
        dial += diff;
        dial = dial.rem_euclid(100);

        if dial == 0 {
            num += 1;
        }
    }

    Ok(num)
}

#[aoc(day01, part2)]
pub fn solve_part2(input: &[Op]) -> Result<i32> {
    let mut dial = 50;
    let mut num = 0;

    for op in input {
        let start = dial;
        let diff = match op {
            Op::Left(v) => -v,
            Op::Right(v) => *v,
        };
        dial += diff;

        num += match (op, start) {
            (Op::Left(_), 0) => -1,
            _ => 0,
        };

        if dial <= 0 {
            num += (dial / 100).abs() + 1;
        }

        if dial >= 100 {
            num += dial / 100;
        }

        dial = dial.rem_euclid(100);
    }

    Ok(num)
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Result<Vec<Op>> {
       let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        input.lines().map(Op::from_str).collect::<Result<Vec<_>>>()
    }

    #[test]
    fn part1() -> Result<()> {
        let input = input()?;

        assert_eq!(solve_part1(&input)?, 3);

        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let input = input()?;

        assert_eq!(solve_part2(&input)?, 6);

        Ok(())
    }
}
