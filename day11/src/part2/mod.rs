mod parse;
use parse::*;

struct MonkeyBusinessIter {
    inner: MonkeyBusiness,
    common_divisor: usize,
    result: Box<[usize]>,
}

impl Iterator for MonkeyBusinessIter {
    type Item = Box<[usize]>;
    fn next(&mut self) -> Option<Self::Item> {
        for i in 0..self.inner.monkeys.len() {
            let inspections = self.inner.monkeys[i].items.len();
            self.result[i] += inspections;
            for j in 0..inspections {
                let monkey = self.inner.monkeys.get(i).expect("safe");
                let worry_level = &monkey.items[j];
                let worry_level = worry_level % self.common_divisor;
                let worry_level_after = monkey.operation.exec(&worry_level);
                let is_divisible = &worry_level_after % monkey.divisible_by == 0;
                let target = if is_divisible {
                    monkey.target
                } else {
                    monkey.fallback
                };

                self.inner.monkeys[target].items.push(worry_level_after);
            }

            self.inner.monkeys.get_mut(i).unwrap().items.clear();
        }
        Some(self.result.clone())
    }
}

impl MonkeyBusiness {
    fn run_rounds(self) -> MonkeyBusinessIter {
        MonkeyBusinessIter {
            common_divisor: self
                .monkeys
                .iter()
                .map(|m| m.divisible_by)
                .product::<usize>(),
            result: vec![0; self.monkeys.len()].into_boxed_slice(),
            inner: self,
        }
    }
}

fn get_inspections_at(i: &str, n: usize) -> <MonkeyBusinessIter as Iterator>::Item {
    parse_monkeys_unsafe(i)
        .run_rounds()
        .nth(n - 1)
        .expect("safe")
}

pub fn solve_part2(input: &str) -> usize {
    let res = get_inspections_at(input, 10_000);
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
    fn test_rounds() {
        assert_eq!(&get_inspections_at(INPUT, 1)[..], &[2, 4, 3, 6]);
        assert_eq!(&get_inspections_at(INPUT, 20)[..], &[99, 97, 8, 103]);
    }
}
