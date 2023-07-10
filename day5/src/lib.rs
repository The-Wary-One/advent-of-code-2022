use std::{collections::VecDeque, str::Lines};

use itertools::Itertools;

struct CratesParser<'a, 'b: 'a> {
    input: &'a mut Lines<'b>,
}

impl<'a, 'b: 'a> CratesParser<'a, 'b> {
    fn new(input: &'a mut Lines<'b>) -> Self {
        Self { input }
    }

    fn parse(&mut self) -> Vec<VecDeque<char>> {
        // Parse the stacks of crates schema.
        let queues = self
            .input
            .take_while_ref(|line| !line.starts_with(" 1"))
            .fold(Vec::<VecDeque<char>>::new(), |mut queues, line| {
                line.char_indices()
                    .filter_map(|(pos, c)| if pos % 4 == 1 { Some(c) } else { None })
                    .enumerate()
                    // Remove empty crates column.
                    .filter(|(_, c)| !c.is_ascii_whitespace())
                    .for_each(|(i, c)| {
                        // Check there is a new crate stack
                        while queues.len() <= i {
                            queues.push(VecDeque::new());
                        }

                        queues[i].push_back(c);
                    });
                queues
            });
        // Verify the number of stacks.
        let nb_stacks = self
            .input
            .next()
            .expect("safe")
            .chars()
            .filter(|c| c.is_numeric())
            .last()
            .and_then(|c| c.to_digit(10))
            .expect("safe");
        assert_eq!(queues.len(), nb_stacks as usize);
        queues
    }
}

struct ProcedureParser<'a, 'b: 'a> {
    input: &'a mut Lines<'b>,
}

impl<'a, 'b: 'a> ProcedureParser<'a, 'b> {
    fn new(input: &'a mut Lines<'b>) -> Self {
        Self { input }
    }

    fn parse(&mut self, moving_fn: impl FnMut((usize, usize, usize))) {
        self.input
            .map(|line| {
                line.split_ascii_whitespace()
                    .filter_map(|word| word.parse::<usize>().ok())
                    .collect_tuple::<(_, _, _)>()
                    .expect("safe")
            })
            .for_each(moving_fn);
    }
}

pub fn solve_part1(mut input: Lines) -> String {
    // Parse and create the crates stacks.
    let mut stacks_parser = CratesParser::new(&mut input);
    let mut crates_stacks = stacks_parser.parse();
    // Remove the empty line
    input.next().expect("safe");
    // Parse and execute the procedure.
    let mut procedure_parser = ProcedureParser::new(&mut input);
    let moving_fn = |(n, from, to)| {
        (0..n).for_each(|_| {
            // Use the VeDeque as a stack here.
            let crat = VecDeque::pop_front(&mut crates_stacks[from - 1]).expect("safe");
            VecDeque::push_front(&mut crates_stacks[to - 1], crat);
        });
    };
    procedure_parser.parse(moving_fn);
    // Get top crates.
    crates_stacks
        .iter()
        .filter_map(|stack| stack.front())
        .collect::<String>()
}

pub fn solve_part2(mut input: Lines) -> String {
    // Parse and create the crates stacks.
    let mut stacks_parser = CratesParser::new(&mut input);
    let mut crates_stacks = stacks_parser.parse();
    // Remove the empty line
    input.next().expect("safe");
    // Parse and execute the procedure.
    let mut procedure_parser = ProcedureParser::new(&mut input);
    let moving_fn = |(n, from, to)| {
        let mut stack = Vec::new();
        (0..n).for_each(|_| {
            // Use the VeDeque as a stack here.
            let crat = VecDeque::pop_front(&mut crates_stacks[from - 1]).expect("safe");
            stack.push(crat);
        });
        while let Some(crat) = stack.pop() {
            VecDeque::push_front(&mut crates_stacks[to - 1], crat);
        }
    };
    procedure_parser.parse(moving_fn);
    // Get top crates.
    crates_stacks
        .iter()
        .filter_map(|stack| stack.front())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT.lines()).as_str(), "CMZ");
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT.lines()), "MCD");
    }
}
