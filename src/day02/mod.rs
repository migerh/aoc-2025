use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{Context, Result, Error};

use crate::utils::AocError;

#[derive(Debug)]
pub struct Range {
    start: i64,
    end: i64,
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spl = s.split('-');
        let start = spl.next().ok_or(AocError::GenericError).context("Could not find start")?.parse::<i64>()?;
        let end = spl.next().ok_or(AocError::GenericError).context("Could not find end")?.parse::<i64>()?;

        Ok(Range { start, end })
    }
}

#[aoc_generator(day02)]
pub fn input_generator(input: &str) -> Result<Vec<Range>> {
    input.split(',').map(Range::from_str).collect::<Result<Vec<_>>>()
}

fn is_invalid(id: i64, div: usize) -> bool {
    let s = id.to_string();
    let l = s.len();

    if !l.is_multiple_of(div) {
        return false;
    }

    let mut subs = vec![];
    let sublen = l / div;

    for i in 0..div {
        subs.push(s.chars().skip(i * sublen).take(sublen).collect::<String>());
    }

    let first = subs[0].clone();
    subs.into_iter().all(|s| s == first)
}

#[aoc(day02, part1)]
pub fn solve_part1(input: &[Range]) -> Result<i64> {
    let mut sum = 0;

    for i in input {
        for k in i.start..=i.end {
            if is_invalid(k, 2) {
                sum += k;
            }
        }
    }

    Ok(sum)
}

fn is_invalid2(id: i64) -> bool {
    let s = id.to_string();
    let l = s.len();

    for i in 2..=l {
        if is_invalid(id, i) {
            return true;
        }
    }

    false
}

#[aoc(day02, part2)]
pub fn solve_part2(input: &[Range]) -> Result<i64> {
    let mut sum = 0;

    for i in input {
        for k in i.start..=i.end {
            if is_invalid2(k) {
                sum += k;
            }
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Result<Vec<Range>> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        input_generator(input)
    }

    #[test]
    fn part1() -> Result<()> {
        let input = input()?;
        assert_eq!(solve_part1(&input)?, 1227775554);

        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let input = input()?;
        assert_eq!(solve_part2(&input)?, 4174379265);

        Ok(())
    }
}