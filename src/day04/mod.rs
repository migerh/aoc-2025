use std::collections::HashMap;

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

type Coords = (i32, i32);
type Map = HashMap<Coords, char>;

#[aoc_generator(day04)]
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

fn count_neighbors(map: &Map, pos: Coords) -> usize {
    let neighbors = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    neighbors.iter().map(|n| {
        let p = (pos.0 + n.0, pos.1 + n.1);
        match map.get(&p) {
            Some('@') => 1,
            _ => 0,
        }
    }).sum()
}

#[aoc(day04, part1)]
pub fn solve_part1(input: &Map) -> Result<usize> {
    let result = input
        .iter()
        .map(|(pos, c)| {
            if *c == '.' {
                0
            } else {
                let count = count_neighbors(input, *pos);
                if count < 4 { 1 } else { 0 }
            }
        })
        .sum::<usize>();

    Ok(result)
}

#[aoc(day04, part2)]
pub fn solve_part2(input: &Map) -> Result<usize> {
    let initial_count = input.iter().filter(|(_, c)| **c == '@').count();
    let mut map = input.clone();
    let mut last_count = initial_count;

    loop {
        for (pos, c) in map.clone() {
            if c == '.' {
                continue;
            }

            let count = count_neighbors(&map, pos);
            if count < 4 {
                map.entry(pos).and_modify(|c| *c = '.');
             }
        }

        let current_count = map.iter().filter(|(_, c)| **c == '@').count();
        if current_count == last_count {
            break;
        }
        last_count = current_count;
    }

    Ok(initial_count - last_count)
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Result<Map> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
        input_generator(input)
    }

    #[test]
    fn part1() -> Result<()> {
        let input = input()?;

        Ok(assert_eq!(solve_part1(&input)?, 13))
    }

    #[test]
    fn part2() -> Result<()> {
        let input = input()?;

        Ok(assert_eq!(solve_part2(&input)?, 43))
    }
}
