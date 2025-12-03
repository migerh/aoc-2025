use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day03)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<u128>>> {
    let result = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as u128)
                .collect::<Vec<_>>()
        })
        .filter(|v| !v.is_empty())
        .collect::<Vec<_>>();
    Ok(result)
}

fn find_largest_naive(battery: &[u128]) -> Result<u128> {
    let l = battery.len();
    let mut max = 0;
    for i in 0..l {
        for j in i + 1..l {
            let candidate = format!("{}{}", battery[i], battery[j]);
            let num = candidate.parse::<u32>()?;

            max = std::cmp::max(num, max);
        }
    }

    Ok(max as u128)
}

#[aoc(day03, part1)]
pub fn solve_part1(input: &[Vec<u128>]) -> Result<u128> {
    let result = input
        .iter()
        .map(|b| find_largest_naive(b))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .sum::<u128>();

    Ok(result)
}

fn find_largest(battery: &[u128], len: usize) -> Result<u128> {
    let l = battery.len();
    let mut num = vec![];
    let mut last_index = 0;


    for i in 0..len {
        let mut max = 0;
        let mut index_of_max = 0;
        for j in last_index..l - (len - 1) + i {
            if battery[j] > max {
                max = battery[j];
                index_of_max = j;
            }
        }

        num.push(battery[index_of_max]);
        last_index = index_of_max + 1;
    }

    let num = num.into_iter().map(|n| n.to_string()).collect::<String>().parse::<u128>()?;

    Ok(num)
}

#[aoc(day03, part2)]
pub fn solve_part2(input: &[Vec<u128>]) -> Result<u128> {
    Ok(input
        .iter()
        .map(|b| find_largest(b, 12))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .sum::<u128>())
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Result<Vec<Vec<u128>>> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        input_generator(input)
    }

    #[test]
    fn part1() -> Result<()> {
        let input = input()?;
        assert_eq!(solve_part1(&input)?, 357);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let input = input()?;
        assert_eq!(solve_part2(&input)?, 3121910778619);
        Ok(())
    }
}
