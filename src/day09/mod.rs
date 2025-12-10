use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{Context, Error, Result};

use crate::utils::AocError;


pub type Base = i64;
#[derive(Debug, Clone)]
pub struct Coords(Base, Base);

impl FromStr for Coords {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split(',')
            .map(|p| Ok(p.parse::<Base>()?))
            .take(2)
            .collect::<Result<Vec<_>>>()?;
        if parts.len() == 2 {
            Ok(Self(parts[0], parts[1]))
        } else {
            Err(AocError::GenericError).context("Could not parse Coords")
        }
    }
}


#[aoc_generator(day09)]
pub fn input_generator(input: &str) -> Result<Vec<Coords>> {
    let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    input.lines().filter(|l| !l.is_empty()).map(Coords::from_str).collect::<Result<Vec<_>>>()
}

fn area(a: &Coords, b: &Coords) -> Base {
    (1 + (b.1 - a.1).abs()) * (1 + (b.0 - a.0).abs())
}

#[aoc(day09, part1)]
pub fn solve_part1(input: &[Coords]) -> Result<Base> {
    let len = input.len();
    let mut max = 0;

    for i in 0..len {
        for j in i+1..len {
            max = std::cmp::max(max, area(&input[i], &input[j]));
        }
    }

    Ok(max)
}

#[aoc(day09, part2)]
pub fn solve_part2(input: &[Coords]) -> Result<i32> {
    let mut edges: Vec<(Coords, Coords)> = vec![];

    for pair in input.windows(2) {
        edges.push((pair[0].clone(), pair[1].clone()));
    }

    // let mut candidates = vec![];
    // for i in 0..len {
    //     for j in i+1..len {
    //         candidates.push((input[i].clone(), input[j].clone(), area(&input[i], &input[j])));
    //     }
    // }
    // candidates.sort_by(|a, b| b.2.cmp(a.2));
    // println!("{:?}", candidates);

    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;
}