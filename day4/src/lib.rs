use std::str::{FromStr, Lines};

use itertools::Itertools;

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

struct CleaningPair(CleaningSection, CleaningSection);

impl FromStr for CleaningPair {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_terminator(',')
            .map(|s| s.parse::<CleaningSection>())
            .try_collect::<_, Vec<CleaningSection>, _>()
            .map(|v| v.into_iter().collect_tuple::<(_, _)>().expect("safe"))
            .map(|(sec1, sec2)| CleaningPair(sec1, sec2))
    }
}

impl CleaningPair {
    fn is_contained(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    fn is_overlapped(&self) -> bool {
        self.0.overlap(&self.1) || self.1.overlap(&self.0)
    }
}

pub fn solve_part1(input: Lines) -> usize {
    input
        .map(|l| l.parse::<CleaningPair>().expect("safe"))
        .filter(|pair| pair.is_contained())
        .count()
}

pub fn solve_part2(input: Lines) -> usize {
    input
        .map(|l| l.parse::<CleaningPair>().expect("safe"))
        .filter(|pair| pair.is_overlapped())
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
