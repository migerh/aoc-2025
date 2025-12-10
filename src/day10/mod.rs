use std::str::FromStr;

use anyhow::{Context, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use microlp::{OptimizationDirection, Problem};

use crate::utils::AocError;

#[derive(Debug, Clone)]
pub struct Machine {
    lights: Vec<char>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl FromStr for Machine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut first = s.split(']');
        let lights = first
            .next()
            .ok_or(AocError::GenericError)
            .context("Could not find light definition")?;
        let buttons_and_joltage = first
            .next()
            .ok_or(AocError::GenericError)
            .context("Could not find buttons and joltage")?;
        let mut rest = buttons_and_joltage.split('{');
        let buttons = rest
            .next()
            .ok_or(AocError::GenericError)
            .context("Could not find buttons")?;
        let joltage = rest
            .next()
            .ok_or(AocError::GenericError)
            .context("Could not find joltage")?;

        let lights = lights.chars().skip(1).collect::<Vec<_>>();
        let buttons = buttons
            .split(' ')
            .filter(|b| !b.is_empty())
            .map(|b| b.replace("(", "").replace(")", ""))
            .map(|b| {
                b.split(',')
                    .map(|n| Ok(n.parse::<usize>()?))
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;
        let joltage = joltage
            .split(',')
            .filter(|j| !j.is_empty())
            .map(|j| j.replace("}", ""))
            .map(|j| Ok(j.parse::<usize>()?))
            .collect::<Result<Vec<_>>>()?;

        Ok(Machine {
            lights,
            buttons,
            joltage,
        })
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Result<Vec<Machine>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(Machine::from_str)
        .collect::<Result<Vec<_>>>()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[Machine]) -> Result<usize> {
    let minima = input.iter().map(|m| {
        let buttons = m.buttons.clone();
        for chunk_size in 1..buttons.len() {
            for combo in buttons.iter().combinations(chunk_size).collect::<Vec<_>>() {
                let mut counts = vec![0; m.lights.len()];
                for toggle in combo {
                    for b in toggle {
                        counts[*b] += 1;
                    }
                }

                if m.lights.iter().enumerate().fold(true, |acc, (pos, c)| acc & match (pos, c) {
                        (pos, '.') => counts[pos] % 2 == 0,
                        (pos, '#') => counts[pos] % 2 == 1,
                        _ => unreachable!(),
                }) {
                    return chunk_size;
                }
            }
        }

        0
    }).collect::<Vec<_>>();

    Ok(minima.into_iter().sum())
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[Machine]) -> Result<f64> {
    let mut minima = vec![];
    for machine in input {
        let buttons = machine.buttons.clone();
        let joltage = machine.joltage.clone();

        let mut problem = Problem::new(OptimizationDirection::Minimize);

        let variables = buttons
            .iter()
            .map(|_| problem.add_integer_var(1.0, (0, i32::MAX)))
            .collect::<Vec<_>>();

        joltage.iter().enumerate().for_each(|(jpos, j)| {
            problem.add_constraint(
                variables
                    .iter()
                    .enumerate()
                    .map(|(bpos, &v)| {
                        let factor = if buttons[bpos].contains(&jpos) {
                            1.0f64
                        } else {
                            0.0f64
                        };

                        (v, factor)
                    })
                    .collect::<Vec<_>>(),
                microlp::ComparisonOp::Eq,
                *j as f64,
            )
        });

        let solution = problem.solve().context("Could not solve ILP")?;
        minima.push(solution.objective());
    }

    Ok(minima.into_iter().sum())
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Result<Vec<Machine>> {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
        input_generator(input)
    }

    #[test]
    fn part1() -> Result<()> {
        let input = input()?;
        Ok(assert_eq!(solve_part1(&input)?, 7))
    }

    #[test]
    fn part2() -> Result<()> {
        let input = input()?;
        Ok(assert_eq!(solve_part2(&input)?, 33.0))
    }
}
