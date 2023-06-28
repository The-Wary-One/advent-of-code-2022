use std::{collections::HashSet, str::Lines};

struct AlphabeticalChar(char);

impl AlphabeticalChar {
    const POSITIONS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    fn position(&self) -> usize {
        Self::POSITIONS.find(self.0).expect("safe")
    }
}

impl TryFrom<char> for AlphabeticalChar {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if Self::POSITIONS.contains(value) {
            Ok(AlphabeticalChar(value))
        } else {
            Err("the char is not in the alphabet")
        }
    }
}

struct Item(AlphabeticalChar);

impl Item {
    fn new(a: AlphabeticalChar) -> Self {
        Self(a)
    }

    fn priority(&self) -> usize {
        self.0.position() + 1
    }
}

pub fn solve_part1(input: Lines) -> usize {
    input
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .map(|(first_compartment, second_compartment)| {
            let first_set = HashSet::<_>::from_iter(first_compartment.chars());
            let second_set = HashSet::from_iter(second_compartment.chars());
            first_set
                .intersection(&second_set)
                .next()
                .expect("safe")
                .to_owned()
        })
        .map(|c| AlphabeticalChar::try_from(c).expect("safe"))
        .map(Item::new)
        .map(|i| i.priority())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_priority() {
        assert_eq!(Item::new('a'.try_into().unwrap()).priority(), 1);
        assert_eq!(Item::new('z'.try_into().unwrap()).priority(), 26);
        assert_eq!(Item::new('A'.try_into().unwrap()).priority(), 27);
        assert_eq!(Item::new('Z'.try_into().unwrap()).priority(), 52);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT.lines()), 157);
    }
}
