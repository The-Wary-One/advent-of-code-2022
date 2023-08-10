use parse::{Packets, Pairs};

pub(crate) mod parse;
pub(crate) mod quick_sort;

pub fn solve_part1(input: &str) -> usize {
    let (_, p) = Pairs::parse(input).expect("safe");
    p.ordered_pairs().map(|(i, _)| i + 1).sum()
}

pub fn solve_part2(input: &str) -> usize {
    let mut input = input.to_owned();
    input.push_str("\n[[2]]\n[[6]]");
    let (_, mut p) = Packets::parse(input.as_str()).expect("safe");
    p.decoder_key()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT), 140);
    }
}
