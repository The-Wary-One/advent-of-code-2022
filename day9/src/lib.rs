use std::str::Lines;

use itertools::Itertools;

pub mod complex;
pub use complex::*;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct Position {
    x: i16,
    y: i16,
}

impl Position {
    fn is_adjacent(&self, other: Self) -> bool {
        self.x.abs_diff(other.x) < 2 && self.y.abs_diff(other.y) < 2
    }

    fn move_to(self, direction: Direction) -> Self {
        use Direction::*;
        match direction {
            Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Right => Position {
                x: self.x + 1,
                y: self.y,
            },
            Up => Position {
                x: self.x,
                y: self.y + 1,
            },
            Down => Position {
                x: self.x,
                y: self.y - 1,
            },
        }
    }
}

pub fn solve_part1(lines: Lines) -> usize {
    let (_, tail_positions) = lines
        .flat_map(|line| {
            let splitted = line.split_ascii_whitespace().collect::<Vec<_>>();
            let steps = splitted[1].parse().expect("safe");
            use Direction::*;
            let direction = match splitted[0] {
                "L" => Left,
                "R" => Right,
                "U" => Up,
                "D" => Down,
                _ => unreachable!(),
            };
            std::iter::repeat(direction).take(steps)
        })
        .fold(
            (Position { x: 0, y: 0 }, vec![Position { x: 0, y: 0 }]),
            |(last_head_pos, mut tail_positions), direction| {
                let new_head_pos = last_head_pos.move_to(direction);
                let last_tail_pos = tail_positions.last().copied().expect("safe");
                if !new_head_pos.is_adjacent(last_tail_pos) {
                    tail_positions.push(last_head_pos);
                }
                (new_head_pos, tail_positions)
            },
        );
    tail_positions.iter().unique().count()
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_part1_simple() {
        assert_eq!(solve_part1(INPUT.lines()), 13);
    }

    #[test]
    fn test_part1_complex() {
        #[cfg(feature = "dhat-heap")]
        let _profiler = dhat::Profiler::new_heap();
        env_logger::init();

        let reader = BufReader::new(INPUT.as_bytes());
        assert_eq!(solve_part1_complex(reader), 13);
    }
}
