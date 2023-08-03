use std::fmt::Display;

use itertools::Itertools;

mod parse;
use parse::*;

pub(crate) struct Round {
    pub(crate) turns: Vec<MonkeyTurn>,
}

pub(crate) struct MonkeyTurn {
    pub(crate) monkey: usize,
    pub(crate) divisible_by: usize,
    pub(crate) inspections: Vec<Inspection>,
}

pub(crate) struct Inspection {
    pub(crate) worry_level: usize,
    pub(crate) operation_result: ExprResult,
    pub(crate) worry_level_after: usize,
    pub(crate) is_divisible: bool,
    pub(crate) target: usize,
}

impl Iterator for MonkeyBusiness {
    type Item = Round;
    fn next(&mut self) -> Option<Self::Item> {
        let mut round = Round { turns: Vec::new() };
        for i in 0..self.monkeys.len() {
            let monkey = self.monkeys.get(i).expect("safe");
            let mut turn = MonkeyTurn {
                monkey: i,
                divisible_by: monkey.divisible_by,
                inspections: Vec::new(),
            };

            for worry_level in monkey.items.iter() {
                let operation_result = monkey.operation.exec(&worry_level);
                let worry_level_during = &operation_result.res;
                let worry_level_after = worry_level_during / 3;
                let is_divisible = &worry_level_after % monkey.divisible_by == 0;
                let target = if is_divisible {
                    monkey.target
                } else {
                    monkey.fallback
                };

                turn.inspections.push(Inspection {
                    worry_level: worry_level.clone(),
                    operation_result,
                    worry_level_after,
                    is_divisible,
                    target,
                });
            }

            for ins in turn.inspections.iter() {
                self.monkeys[ins.target]
                    .items
                    .push(ins.worry_level_after.clone());
            }
            self.monkeys.get_mut(i).unwrap().items.clear();

            round.turns.push(turn);
        }
        Some(round)
    }
}

impl Display for Round {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let o = self.turns.iter().join("\n");
        write!(f, "{}", o)
    }
}

impl Display for MonkeyTurn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .inspections
            .iter()
            .map(|ins| {
                let Inspection {
                    worry_level,
                    operation_result,
                    worry_level_after,
                    is_divisible,
                    target,
                } = ins;
                let divisible = if *is_divisible {
                    "divisible"
                } else {
                    "not divisible"
                };
                let divisible_by = self.divisible_by;
                format!(
                    "  Monkey inspects an item with a worry level of {worry_level}.
    {operation_result}.
    Monkey gets bored with item. Worry level is divided by 3 to {worry_level_after}.
    Current worry level is {divisible} by {divisible_by}.
    Item with worry level {worry_level_after} is thrown to monkey {target}."
                )
            })
            .join("\n");
        let monkey = self.monkey;
        write!(f, "Monkey {monkey}:\n{s}")
    }
}

impl Display for ExprResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Op::*;
        let op = match self.op {
            Add => "increases",
            Mul => "is multiplied",
        };
        use Value::*;
        let by = match self.by {
            Constant(c) => c.to_string(),
            Input => "itself".to_owned(),
        };
        let res = self.res.clone();
        write!(f, "Worry level {op} by {by} to {res}")
    }
}

pub fn solve_part1(input: &str) -> usize {
    let m = parse_monkeys_unsafe(input);
    let len = m.monkeys.len();
    let res = m
        .take(20)
        .fold(vec![0; len].into_boxed_slice(), |mut acc, round| {
            for turn in round.turns {
                acc[turn.monkey] += turn.inspections.len();
            }
            acc
        });
    let max1 = *res.iter().max().expect("safe");
    let max2 = *res.iter().filter(|x| **x != max1).max().expect("safe");
    max1 * max2
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_round() {
        let expected = "Monkey 0:
  Monkey inspects an item with a worry level of 79.
    Worry level is multiplied by 19 to 1501.
    Monkey gets bored with item. Worry level is divided by 3 to 500.
    Current worry level is not divisible by 23.
    Item with worry level 500 is thrown to monkey 3.
  Monkey inspects an item with a worry level of 98.
    Worry level is multiplied by 19 to 1862.
    Monkey gets bored with item. Worry level is divided by 3 to 620.
    Current worry level is not divisible by 23.
    Item with worry level 620 is thrown to monkey 3.
Monkey 1:
  Monkey inspects an item with a worry level of 54.
    Worry level increases by 6 to 60.
    Monkey gets bored with item. Worry level is divided by 3 to 20.
    Current worry level is not divisible by 19.
    Item with worry level 20 is thrown to monkey 0.
  Monkey inspects an item with a worry level of 65.
    Worry level increases by 6 to 71.
    Monkey gets bored with item. Worry level is divided by 3 to 23.
    Current worry level is not divisible by 19.
    Item with worry level 23 is thrown to monkey 0.
  Monkey inspects an item with a worry level of 75.
    Worry level increases by 6 to 81.
    Monkey gets bored with item. Worry level is divided by 3 to 27.
    Current worry level is not divisible by 19.
    Item with worry level 27 is thrown to monkey 0.
  Monkey inspects an item with a worry level of 74.
    Worry level increases by 6 to 80.
    Monkey gets bored with item. Worry level is divided by 3 to 26.
    Current worry level is not divisible by 19.
    Item with worry level 26 is thrown to monkey 0.
Monkey 2:
  Monkey inspects an item with a worry level of 79.
    Worry level is multiplied by itself to 6241.
    Monkey gets bored with item. Worry level is divided by 3 to 2080.
    Current worry level is divisible by 13.
    Item with worry level 2080 is thrown to monkey 1.
  Monkey inspects an item with a worry level of 60.
    Worry level is multiplied by itself to 3600.
    Monkey gets bored with item. Worry level is divided by 3 to 1200.
    Current worry level is not divisible by 13.
    Item with worry level 1200 is thrown to monkey 3.
  Monkey inspects an item with a worry level of 97.
    Worry level is multiplied by itself to 9409.
    Monkey gets bored with item. Worry level is divided by 3 to 3136.
    Current worry level is not divisible by 13.
    Item with worry level 3136 is thrown to monkey 3.
Monkey 3:
  Monkey inspects an item with a worry level of 74.
    Worry level increases by 3 to 77.
    Monkey gets bored with item. Worry level is divided by 3 to 25.
    Current worry level is not divisible by 17.
    Item with worry level 25 is thrown to monkey 1.
  Monkey inspects an item with a worry level of 500.
    Worry level increases by 3 to 503.
    Monkey gets bored with item. Worry level is divided by 3 to 167.
    Current worry level is not divisible by 17.
    Item with worry level 167 is thrown to monkey 1.
  Monkey inspects an item with a worry level of 620.
    Worry level increases by 3 to 623.
    Monkey gets bored with item. Worry level is divided by 3 to 207.
    Current worry level is not divisible by 17.
    Item with worry level 207 is thrown to monkey 1.
  Monkey inspects an item with a worry level of 1200.
    Worry level increases by 3 to 1203.
    Monkey gets bored with item. Worry level is divided by 3 to 401.
    Current worry level is not divisible by 17.
    Item with worry level 401 is thrown to monkey 1.
  Monkey inspects an item with a worry level of 3136.
    Worry level increases by 3 to 3139.
    Monkey gets bored with item. Worry level is divided by 3 to 1046.
    Current worry level is not divisible by 17.
    Item with worry level 1046 is thrown to monkey 1.";
        let mut m = parse_monkeys_unsafe(INPUT);
        assert_eq!(m.next().unwrap().to_string(), expected);
    }
}
