use std::{
    cmp::Ordering,
    str::{FromStr, Lines},
};

use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd)]
enum Choice {
    Rock = 1,
    Paper,
    Scissors,
}

impl TryFrom<char> for Choice {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use Choice::*;
        match c {
            'A' | 'X' => Ok(Rock),
            'B' | 'Y' => Ok(Paper),
            'C' | 'Z' => Ok(Scissors),
            _ => Err("Unexpected char"),
        }
    }
}

impl FromStr for Choice {
    type Err = <Choice as TryFrom<char>>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .next()
            .ok_or_else(|| "Empty string")
            .and_then(|c| c.try_into())
    }
}

impl Ord for Choice {
    fn cmp(&self, other: &Self) -> Ordering {
        use Choice::*;
        match (self, other) {
            // gt
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Ordering::Greater,
            // lt
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Ordering::Less,
            // eq
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Ordering::Equal,
        }
    }
}

struct Round(Choice, Choice);

impl Round {
    fn new(p1: Choice, p2: Choice) -> Self {
        Round(p1, p2)
    }

    fn play(self) -> usize {
        match self.0.cmp(&self.1) {
            Ordering::Less => self.0 as usize,
            Ordering::Equal => 3 + self.0 as usize,
            Ordering::Greater => 6 + self.0 as usize,
        }
    }
}

impl From<(Choice, Choice)> for Round {
    fn from(value: (Choice, Choice)) -> Self {
        Round::new(value.0, value.1)
    }
}

pub fn solve_part1(input: Lines) -> usize {
    input
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse::<Choice>().expect("safe"))
                .rev()
                .collect_tuple::<(_, _)>()
                .expect("safe")
                .into()
        })
        .map(|r: Round| r.play())
        .sum()
}

enum RoundResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl TryFrom<char> for RoundResult {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use RoundResult::*;
        match value {
            'X' => Ok(Loss),
            'Y' => Ok(Draw),
            'Z' => Ok(Win),
            _ => Err("Unexpected char"),
        }
    }
}

impl FromStr for RoundResult {
    type Err = <RoundResult as TryFrom<char>>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .next()
            .ok_or_else(|| "Empty string")
            .and_then(|c| c.try_into())
    }
}

pub fn solve_part2(input: Lines) -> usize {
    input
        .map(|l| {
            l.split_ascii_whitespace()
                .collect_tuple::<(_, _)>()
                .expect("safe")
        })
        .map(|(c, r)| (c.parse().expect("safe"), r.parse().expect("safe")))
        .map(|(them, res)| {
            use Choice::*;
            use RoundResult::*;
            let to_play = match (&them, &res) {
                (Rock, Draw) | (Paper, Loss) | (Scissors, Win) => Rock,
                (Rock, Win) | (Paper, Draw) | (Scissors, Loss) => Paper,
                (Rock, Loss) | (Paper, Win) | (Scissors, Draw) => Scissors,
            };
            Round(to_play, them)
        })
        .map(Round::play)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT.lines()), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT.lines()), 12);
    }
}
