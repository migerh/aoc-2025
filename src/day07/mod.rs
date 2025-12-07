use std::{collections::{HashMap, HashSet}};

use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{Result, Context};
use memoize::memoize;

use crate::utils::AocError;

type Coords = (i32, i32);
type Map = HashMap<Coords, char>;

#[aoc_generator(day07)]
pub fn input_generator(input: &str) -> Result<Map> {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c))
        })
        .collect::<HashMap<_, _>>();

    Ok(map)
}

fn get_start(map: &Map) -> Option<i32> {
    map.iter().filter(|(_, v)| **v == 'S').map(|(c, _)| c).next().map(|c| c.1)
}

fn get_num_layers(map: &Map) -> Option<i32> {
    map.iter().map(|(c, _)| c.0).max()
}

fn split_beams(map: &Map, layer: i32, beams: &Vec<i32>) -> (i32, Vec<i32>) {
    let splitters = map.iter().filter(|(c, v)| c.0 == layer && beams.contains(&c.1) && **v == '^').map(|(c, _)| c.1).collect::<Vec<_>>();
    let num_new_splits = splitters.len();
    let continuing_beams = beams.iter().filter(|p| !splitters.contains(p)).cloned().collect::<HashSet<_>>();
    let split_beams = splitters.into_iter().flat_map(|p| vec![p-1, p+1]).collect::<HashSet<_>>();
    let result = continuing_beams.union(&split_beams).cloned().collect::<Vec<_>>();

    (num_new_splits as i32, result)
}

#[aoc(day07, part1)]
pub fn solve_part1(input: &Map) -> Result<i32> {
    let start = get_start(input).ok_or(AocError::GenericError).context("Could not find start")?;
    let layers= get_num_layers(input).ok_or(AocError::GenericError).context("Could not count layers")?;
    let mut splits = 0;
    let mut beams = vec![start];

    for l in 1..layers {
        let (s, new_beams) = split_beams(&input, l, &beams);
        splits += s;
        beams = new_beams;
    }

    Ok(splits)
}

#[memoize(Ignore: map, Ignore: max)]
fn split_beams3(map: &Map, max: i32, row: i32, col: i32) -> u128 {
    if row >= max {
        return 1;
    }

    let result = match map.get(&(row, col)) {
        Some('^') => split_beams3(map, max, row+1, col - 1) + split_beams3(map, max, row+1, col + 1),
        _ => split_beams3(map, max, row+1, col),
    };

    result
}

#[aoc(day07, part2)]
pub fn solve_part2(input: &Map) -> Result<u128> {
    let start = get_start(input).ok_or(AocError::GenericError).context("Could not find start")?;
    let layers= get_num_layers(input).ok_or(AocError::GenericError).context("Could not count layers")?;

    Ok(split_beams3(&input, layers, 1, start))
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Result<Map> {
    let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
        input_generator(input)
    }

    #[test]
    fn part1() -> Result<()> {
        let input = input()?;
        Ok(assert_eq!(solve_part1(&input)?, 21))
    }

    #[test]
    fn part2() -> Result<()> {
        let input = input()?;
        Ok(assert_eq!(solve_part2(&input)?, 40))
    }
}