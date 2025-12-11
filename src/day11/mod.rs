use std::collections::HashMap;

use anyhow::{Context, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use memoize::memoize;

use crate::utils::AocError;

type Graph = HashMap<String, Vec<String>>;

fn parse_line(s: &str) -> Result<(String, Vec<String>)> {
    let mut split = s.split(":");
    let left = split
        .next()
        .ok_or(AocError::GenericError)
        .context("Could not find lhs")?
        .to_string();
    let right = split
        .next()
        .ok_or(AocError::GenericError)
        .context("Could not find rhs")?;

    let right = right
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    Ok((left, right))
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Result<Graph> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(parse_line)
        .collect::<Result<Graph>>()
}

fn find_all(graph: &Graph, start: String, end: String) -> Option<Vec<Vec<String>>> {
    let mut todo = vec![vec![start]];
    let mut results = vec![];

    while let Some(node) = todo.pop() {
        let last = node.last()?;
        let next = graph.get(last);
        if next.is_none() {
            continue;
        }
        let next = next.unwrap();

        for n in next {
            let mut path = node.clone();
            if path.contains(&n) {
                continue;
            }

            path.push(n.to_string());

            if *n == end {
                results.push(path);
            } else {
                todo.push(path);
            }
        }
    }

    Some(results)
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Graph) -> Result<usize> {
    Ok(find_all(input, "you".to_string(), "out".to_string())
        .ok_or(AocError::GenericError)
        .context("Could not find all paths")?
        .len())
}

#[memoize(Ignore: graph)]
fn count_all(graph: &Graph, start: String, end: String) -> usize {
    if start == end {
        return 1;
    }

    let neighbors = graph.get(&start);
    if neighbors.is_none() {
        return 0;
    }

    let neighbors = neighbors.unwrap();
    let mut paths = 0;
    for n in neighbors {
        paths += count_all(graph, n.clone(), end.clone());
    }
    paths
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Graph) -> Result<usize> {
    let fft_out = count_all(input, "fft".to_string(), "out".to_string());
    let dac_out = count_all(input, "dac".to_string(), "out".to_string());
    let fft_dac = count_all(input, "fft".to_string(), "dac".to_string());
    let dac_fft = count_all(input, "dac".to_string(), "fft".to_string());
    let svr_dac = count_all(input, "svr".to_string(), "dac".to_string());
    let svr_fft = count_all(input, "svr".to_string(), "fft".to_string());

    Ok(svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";
        let input = input_generator(input)?;
        Ok(assert_eq!(solve_part1(&input)?, 5))
    }

    #[test]
    fn part2() -> Result<()> {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";
        let input = input_generator(input)?;
        Ok(assert_eq!(solve_part2(&input)?, 2))
    }
}
