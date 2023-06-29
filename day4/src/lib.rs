use std::str::{FromStr, Lines};

use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
struct CleaningSection {
    start: usize,
    end: usize,
}

impl FromStr for CleaningSection {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_terminator('-')
            .map(|s| s.parse::<usize>())
            .try_collect::<_, Vec<usize>, _>()
            .map(|v| v.into_iter().collect_tuple::<(_, _)>().expect("safe"))
            .map(|(start, end)| CleaningSection { start, end })
    }
}

impl CleaningSection {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlap(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.start
    }
}

macro_rules! get_cleaning_section_pair {
    ($input:expr) => {
        $input.map(|l| {
            l.split_terminator(',')
                .map(|s| s.parse::<CleaningSection>().expect("safe"))
                .collect_tuple::<(_, _)>()
                .expect("safe")
        })
    };
}

pub fn solve_part1(input: Lines) -> usize {
    get_cleaning_section_pair!(input)
        .filter(|(sec1, sec2)| sec1.contains(&sec2) || sec2.contains(&sec1))
        .count()
}

pub fn solve_part2(input: Lines) -> usize {
    get_cleaning_section_pair!(input)
        .filter(|(sec1, sec2)| sec1.overlap(&sec2) || sec2.overlap(&sec1))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT.lines()), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT.lines()), 4);
    }
}
