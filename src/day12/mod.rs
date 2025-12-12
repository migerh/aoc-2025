use std::str::FromStr;

use anyhow::{Context, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::AocError;

type Shape = Vec<Vec<char>>;

fn parse_shape(s: &str) -> Result<Shape> {
    Ok(s.lines()
        .filter(|l| !l.is_empty())
        .skip(1)
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>())
}

#[derive(Debug, Clone)]
pub struct Tree {
    width: usize,
    height: usize,
    presents: Vec<usize>,
}

impl FromStr for Tree {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut split = s.split(":");
        let size = split
            .next()
            .ok_or(AocError::GenericError)?
            .split("x")
            .map(|n| Ok(n.parse::<usize>()?))
            .collect::<Result<Vec<_>>>()
            .context("Could not parse tree size")?;

        if size.len() != 2 {
            return Err(AocError::GenericError).context("Tree size is invalid");
        }

        let width = size[0];
        let height = size[1];

        let presents = split
            .next()
            .ok_or(AocError::GenericError)?
            .split_whitespace()
            .filter(|v| !v.is_empty())
            .map(|v| Ok(v.parse::<usize>()?))
            .collect::<Result<Vec<_>>>()
            .context("Could not parse present indices")?;

        Ok(Self {
            width,
            height,
            presents,
        })
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Result<(Vec<Shape>, Vec<Tree>)> {
//     let input = "0:
// ###
// ##.
// ##.

// 1:
// ###
// ##.
// .##

// 2:
// .##
// ###
// ##.

// 3:
// ##.
// ###
// ##.

// 4:
// ###
// #..
// ###

// 5:
// ###
// .#.
// ###

// 4x4: 0 0 0 0 2 0
// 12x5: 1 0 1 0 2 2
// 12x5: 1 0 1 0 3 2
// ";
    let split = input
        .split("\n\n")
        .filter(|e| !e.is_empty())
        .collect::<Vec<_>>();

    let trees = split
        .iter()
        .rev()
        .next()
        .ok_or(AocError::GenericError)
        .context("Could not find trees")?
        .lines()
        .filter(|l| !l.is_empty())
        .map(Tree::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Could not parse trees")?;
    let shapes = split
        .iter()
        .rev()
        .skip(1)
        .rev()
        .cloned()
        .map(parse_shape)
        .collect::<Result<Vec<_>>>()
        .context("Could not parse shapes")?;

    Ok((shapes, trees))
}

fn count_spaces(s: &Shape) -> usize {
    s.iter().map(|l| l.iter().filter(|&&c| c == '#').count()).sum()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &(Vec<Shape>, Vec<Tree>)) -> Result<usize> {
    let (shapes, trees) = input;

    let spaces = shapes.iter().map(count_spaces).collect::<Vec<_>>();
    println!("{:?}", spaces);

    let mut count = 0;
    for tree in trees {
        let available = tree.width * tree.height;
        let required: usize = tree.presents.iter().enumerate().filter_map(|(p, c)| Some(c * spaces.get(p)?)).sum();
        println!("available {available}");
        println!("required {required}");
        println!();
        if required <= available {
            count += 1;
        }
    }

    Ok(count)

    // 1000: no hint
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &(Vec<Shape>, Vec<Tree>)) -> Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;
}
