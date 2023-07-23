use std::{
    ops::{Add, AddAssign, Sub},
    str::Lines,
};

use itertools::Itertools;

pub mod complex;
pub use complex::*;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Default, Debug)]
pub struct Position {
    x: i16,
    y: i16,
}

impl Position {
    #[inline(always)]
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub fn x(&self) -> i16 {
        self.x
    }

    #[inline(always)]
    pub fn y(&self) -> i16 {
        self.y
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }
    #[inline(always)]
    pub fn is_adjacent(&self, other: Self) -> bool {
        self.x.abs_diff(other.x) < 2 && self.y.abs_diff(other.y) < 2
    }

    #[inline(always)]
    pub fn move_to(self, direction: Direction) -> Self {
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

    pub fn move_delta(self, delta: Position) -> Self {
        let (dx, dy) = match (delta.x, delta.y) {
            // overlapping
            (0, 0) => (0, 0),
            // touching up/left/down/right
            (0, 1) | (1, 0) | (0, -1) | (-1, 0) => (0, 0),
            // touching diagonally
            (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),
            // need to move up/left/down/right
            (0, 2) => (0, 1),
            (0, -2) => (0, -1),
            (2, 0) => (1, 0),
            (-2, 0) => (-1, 0),
            // need to move to the right diagonally
            (2, 1) => (1, 1),
            (2, -1) => (1, -1),
            // need to move to the left diagonally
            (-2, 1) => (-1, 1),
            (-2, -1) => (-1, -1),
            // need to move up/down diagonally
            (1, 2) => (1, 1),
            (-1, 2) => (-1, 1),
            (1, -2) => (1, -1),
            (-1, -2) => (-1, -1),
            // 🆕 need to move diagonally
            (-2, -2) => (-1, -1),
            (-2, 2) => (-1, 1),
            (2, -2) => (1, -1),
            (2, 2) => (1, 1),
            _ => panic!("unhandled case: {delta:?}"),
        };
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
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
