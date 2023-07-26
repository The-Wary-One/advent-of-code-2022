use std::{
    fmt::Display,
    io::{BufRead, Lines},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, space1},
    combinator::map,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn cycles(&self) -> usize {
        use Instruction::*;
        match self {
            Noop => 1,
            Addx(_) => 2,
        }
    }
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(tag("noop"), |_| Instruction::Noop),
        map(separated_pair(tag("addx"), space1, i32), |(_, n)| {
            Instruction::Addx(n)
        }),
    ))(input)
}

pub fn solve_part1(lines: Lines<impl BufRead>) -> i32 {
    let (_, _, res) = lines
        .map(|res| {
            let line = res.expect("safe");
            let (_, ins) = parse_instruction(line.as_str()).expect("safe");
            ins
        })
        .fold(
            (0_u16, 1_i32, 0_i32),
            |(mut cycle, mut x, mut res), instruction| {
                for _ in 0..instruction.cycles() {
                    cycle += 1;

                    if (cycle + 20) % 40 == 0 {
                        res += i32::from(cycle) * x;
                    }
                }

                if let Instruction::Addx(n) = instruction {
                    x += n;
                }

                (cycle, x, res)
            },
        );
    res
}

#[repr(transparent)]
pub struct CRT([u64; 4]);

impl CRT {
    const HEIGHT: u8 = 6;
    const WIDTH: u8 = 40;

    fn new() -> Self {
        Self([0; 4])
    }

    //fn get_pixel(&self, x: u8, y: u8) -> Result<bool, &str> {
    //    if x >= Self::WIDTH || y >= Self::HEIGHT {
    //        return Err("invalid pixel");
    //    }

    //    let position = y * Self::WIDTH + x;
    //    let index: usize = (position / 64).into();
    //    let shift = position % 64;
    //    Ok((self.0[index] >> shift) & 1 == 1)
    //}

    fn get_line(&self, y: u8) -> Result<[bool; Self::WIDTH as usize], &str> {
        if y >= Self::HEIGHT {
            return Err("invalid pixel line");
        }

        let mut arr = [false; Self::WIDTH as usize];
        let start = y * Self::WIDTH;
        for (line_pos, bit_pos) in (start..start + Self::WIDTH).enumerate() {
            let index: usize = (bit_pos / 64).into();
            let shift = bit_pos % 64;
            let bit = (self.0[index] >> shift) & 1 == 1;
            arr[line_pos] = bit;
        }
        Ok(arr)
    }

    fn draw_pixel(&mut self, x: u8, y: u8) -> Result<(), &str> {
        if x >= Self::WIDTH || y >= Self::HEIGHT {
            return Err("invalid pixel");
        }

        let position = y * Self::WIDTH + x;
        let index: usize = (position / 64) as usize;
        let shift = position % 64;
        self.0[index] |= 1 << shift;

        Ok(())
    }
}

impl Display for CRT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::with_capacity((Self::HEIGHT * Self::WIDTH) as usize);
        for l in 0..Self::HEIGHT {
            if l != 0 {
                s.push('\n');
            }
            self.get_line(l)
                .expect("safe")
                .into_iter()
                .map(|b| if b { '#' } else { '.' })
                .for_each(|c| s.push(c));
        }
        write!(f, "{}", s)
    }
}

pub fn solve_part2(lines: Lines<impl BufRead>) -> String {
    let (_, _, crt) = lines
        .map(|res| {
            let line = res.expect("safe");
            let (_, ins) = parse_instruction(line.as_str()).expect("safe");
            ins
        })
        .fold(
            (0_u8, 1_i32, CRT::new()),
            |(mut cycle, mut x, mut crt), instruction| {
                for _ in 0..instruction.cycles() {
                    let pos_y = cycle / CRT::WIDTH;
                    let pos_x = cycle % CRT::WIDTH;
                    cycle += 1;

                    if i32::from(pos_x) >= x - 1 && i32::from(pos_x) <= x + 1 {
                        crt.draw_pixel(pos_x, pos_y).expect("safe");
                    }
                }

                if let Instruction::Addx(n) = instruction {
                    x += n;
                }

                (cycle, x, crt)
            },
        );
    crt.to_string()
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_part1() {
        let reader = BufReader::new(INPUT.as_bytes());
        assert_eq!(solve_part1(reader.lines()), 13140);
    }

    #[test]
    fn test_part2() {
        let reader = BufReader::new(INPUT.as_bytes());
        assert_eq!(
            solve_part2(reader.lines()),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
