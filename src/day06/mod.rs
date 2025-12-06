use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::AocError;

#[derive(Debug)]
pub enum Op {
    Add(Vec<i128>),
    Mul(Vec<i128>),
}

impl Op {
    fn from_column(table: &Vec<Vec<&str>>, column: usize) -> Option<Self> {
        use Op::*;

        let mut col = table
            .iter()
            .filter_map(|c| c.get(column))
            .cloned()
            .collect::<Vec<_>>();
        let op = col.pop()?;
        let col = col.into_iter().map(|v| Ok(v.parse::<i128>()?)).collect::<Result<Vec<_>>>().ok()?;

        Some(match op {
            "+" => Add(col),
            "*" => Mul(col),
            _ => None?
        })
    }

    fn calculate(&self) -> i128 {
        use Op::*;

        let result = match self {
            Add(v) => v.iter().sum::<i128>(),
            Mul(v) => v.iter().product::<i128>(),
        };

        result
    }

    fn from_part2(cols: &Vec<Vec<char>>, op: &str) -> Result<Op> {
        use Op::*;

        let nums = cols.iter().map(|c| Ok(c.iter().collect::<String>().trim().parse::<i128>()?)).collect::<Result<Vec<_>>>()?;

        Ok(match op {
            "+" => Add(nums),
            "*" => Mul(nums),
            _ => Err(AocError::GenericError)?
        })
    }
}

#[aoc_generator(day06)]
pub fn input_generator(input: &str) -> Result<String> {
    Ok(input.to_string())
}

fn parse_part1(input: &str) -> Result<Vec<Op>> {
    let input = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split(' ').filter(|c| !c.is_empty()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let num_cols = input[0].len();

    Ok((0..num_cols).into_iter().filter_map(|c| Op::from_column(&input, c)).collect::<Vec<_>>())
}

#[aoc(day06, part1)]
pub fn solve_part1(input: &str) -> Result<i128> {
    let input = parse_part1(input)?;
    Ok(input.iter().map(|o| o.calculate()).sum::<i128>())
}

fn parse_part2(input: &str) -> Result<Vec<Op>> {
    let mut lines = input.lines().filter(|l| !l.is_empty()).collect::<Vec<_>>();
    let ops = lines.pop().ok_or(AocError::GenericError)?.split(' ').filter(|s| !s.is_empty()).collect::<Vec<_>>();
    let max_cols = lines.iter().map(|l| l.len()).max().ok_or(AocError::GenericError)?;

    let mut cols = vec![];
    let mut col = vec![];
    for i in 0..max_cols {
        let c = max_cols - 1 - i;

        let col_line = lines.iter().map(|l| l.chars().collect::<Vec<_>>().get(c).cloned().unwrap_or(' ')).collect::<Vec<_>>();
        if col_line.iter().all(|ch| *ch == ' ') {
            cols.push(col);
            col = vec![];
        } else {
            col.push(col_line);
        }
    }
    if col.len() != 0 {
        cols.push(col);
    }

    assert_eq!(ops.len(), cols.len());

    let mut result = vec![];
    for i in 0..ops.len() {
        result.push(Op::from_part2(&cols[i], ops[ops.len() - i - 1])?);
    }

    Ok(result)
}

#[aoc(day06, part2)]
pub fn solve_part2(input: &str) -> Result<i128> {
    let input = parse_part2(input)?;
    Ok(input.iter().map(|o| o.calculate()).sum::<i128>())
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> String {
    "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
".to_string()
    }

    #[test]
    fn part_1() -> Result<()> {
        let input = input();
        Ok(assert_eq!(solve_part1(&input)?, 4277556))
    }

    #[test]
    fn part_2() -> Result<()> {
        let input = input();
        Ok(assert_eq!(solve_part2(&input)?, 3263827))
    }
}
