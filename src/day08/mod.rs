use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use anyhow::{Context, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::utils::AocError;

pub type Base = i64;
pub struct Coords(Base, Base, Base);

impl FromStr for Coords {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split(',')
            .map(|p| Ok(p.parse::<Base>()?))
            .take(3)
            .collect::<Result<Vec<_>>>()?;
        if parts.len() == 3 {
            Ok(Self(parts[0], parts[1], parts[2]))
        } else {
            Err(AocError::GenericError).context("Could not parse Coords")
        }
    }
}

#[aoc_generator(day08)]
pub fn input_generator(input: &str) -> Result<Vec<Coords>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(Coords::from_str)
        .collect::<Result<Vec<_>>>()
}

fn distance(a: &Coords, b: &Coords) -> f64 {
    f64::sqrt(((b.0 - a.0).pow(2) + (b.1 - a.1).pow(2) + (b.2 - a.2).pow(2)) as f64)
}

fn disjunct_networks(paths: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut networks: Vec<Vec<usize>> = vec![];
    let mut done = vec![false; paths.len()];

    while done.iter().any(|v| !v) {
        let first = done.iter().find_position(|v| !**v).unwrap().0;

        let mut network = HashSet::new();
        network.insert(paths[first].0);
        network.insert(paths[first].1);

        let mut old = 0;
        while old != network.len() {
            old = network.len();
            for i in 0..paths.len() {
                if !done[i] && (network.contains(&paths[i].0) || network.contains(&paths[i].1)) {
                    done[i] = true;
                    network.insert(paths[i].0);
                    network.insert(paths[i].1);
                }
            }
        }

        networks.push(network.into_iter().collect::<Vec<_>>());
    }

    networks
}

#[aoc(day08, part1)]
pub fn solve_part1(input: &[Coords]) -> Result<usize> {
    let num = if input.len() < 50 {
        10
    } else {
        1000
    };
    let mut pairs = vec![];
    let len = input.len();

    for i in 0..len {
        for j in i + 1..len {
            pairs.push((i, j, distance(&input[i], &input[j])));
        }
    }

    pairs.sort_by(|a, b| {
        if a.2 < b.2 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    let pairs = pairs.into_iter().map(|p| (p.0, p.1)).take(num).collect::<Vec<_>>();
    let mut networks = disjunct_networks(&pairs);

    networks.sort_by_key(|a| a.len());
    Ok(networks.into_iter().rev().take(3).map(|n| n.len()).product())
}

fn find_last_connection(pairs: VecDeque<(usize, usize)>, max: usize) -> Option<(usize, usize)> {
    let mut connections = pairs.iter().take(max).cloned().collect::<Vec<_>>();
    let mut pairs = pairs.into_iter().skip(max).collect::<VecDeque<_>>();
    while let Some(first) = pairs.pop_front() {
        connections.push((first.0, first.1));
        let mut networks = disjunct_networks(&connections);
        networks.sort_by_key(|a| a.len());

        if networks[0].len() == max {
            return Some((first.0, first.1));
        }
    }

    None
}

#[aoc(day08, part2)]
pub fn solve_part2(input: &[Coords]) -> Result<Base> {
    let mut pairs = vec![];
    let len = input.len();

    for i in 0..len {
        for j in i + 1..len {
            pairs.push((i, j, distance(&input[i], &input[j])));
        }
    }

    pairs.sort_by(|a, b| {
        if a.2 < b.2 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    let pairs = pairs.into_iter().map(|p| (p.0, p.1)).collect::<VecDeque<_>>();

    if let Some(last) = find_last_connection(pairs, input.len()) {
        Ok(input[last.0].0 * input[last.1].0)
    } else {
        Err(AocError::GenericError).context("Could not connect all junction boxes")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Result<Vec<Coords>> {
    let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
        input_generator(input)
    }

    #[test]
    fn part1() -> Result<()> {
        let input = input()?;
        Ok(assert_eq!(solve_part1(&input)?, 40))
    }

    #[test]
    fn part2() -> Result<()> {
        let input = input()?;
        Ok(assert_eq!(solve_part2(&input)?, 25272))
    }
}
