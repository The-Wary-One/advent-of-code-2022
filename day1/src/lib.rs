use std::str::Lines;

use itertools::Itertools;

macro_rules! get_calories_per_elf {
    ($input:expr) => {
        $input
            .into_iter()
            .group_by(|l| !l.is_empty())
            .into_iter()
            .filter_map(|(key, group)| {
                if key {
                    group
                        .into_iter()
                        .map(|x| x.parse::<usize>().expect("unexpected"))
                        .sum1::<usize>()
                } else {
                    None
                }
            })
    };
}

pub fn solve_part1(input: Lines) -> usize {
    get_calories_per_elf!(input).max().expect("safe")
}

pub fn solve_part2(input: Lines) -> usize {
    get_calories_per_elf!(input).sorted().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT.lines()), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT.lines()), 45000);
    }
}
