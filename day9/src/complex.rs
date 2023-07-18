use std::{collections::VecDeque, io::Read};

use itertools::Itertools;
use log::debug;
use nom::{
    character::complete,
    character::streaming::{line_ending, one_of, space1, u32},
    combinator::{complete, map},
    error::{convert_error, VerboseError},
    multi::many1,
    sequence::{separated_pair, terminated},
    Err, IResult,
};
use smallvec::SmallVec;

use crate::{Direction, Position};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static GLOBAL: dhat::Alloc = dhat::Alloc;

fn parse_direction(input: &str) -> IResult<&str, Direction, VerboseError<&str>> {
    use Direction::*;
    map(one_of("LRUD"), |c| match c {
        'L' => Left,
        'R' => Right,
        'U' => Up,
        'D' => Down,
        _ => unreachable!(),
    })(input)
}

fn parse_number(input: &str) -> IResult<&str, usize, VerboseError<&str>> {
    map(u32, |n| n as usize)(input)
}

fn parse_command(input: &str) -> IResult<&str, (Direction, usize), VerboseError<&str>> {
    complete(terminated(
        separated_pair(parse_direction, space1, parse_number),
        line_ending,
    ))(input)
}

fn parse_commands(input: &str) -> IResult<&str, Vec<(Direction, usize)>, VerboseError<&str>> {
    many1(parse_command)(input)
}

fn parse_command_end(input: &str) -> IResult<&str, (Direction, usize), VerboseError<&str>> {
    use Direction::*;
    //preceded(
    //    opt(newline),
    separated_pair(
        map(complete::one_of("LRUD"), |c| match c {
            'L' => Left,
            'R' => Right,
            'U' => Up,
            'D' => Down,
            _ => unreachable!(),
        }),
        complete::space1,
        map(complete::u32, |n| n as usize),
        //    ),
    )(input)
}

struct DirectionReaderIterator<B: Read, const N: usize> {
    inner: B,
    buffer: [u8; N],
    // TODO: use smallvec.
    acc: Option<Vec<u8>>,
    res_queue: VecDeque<Direction>,
}

impl<B: Read, const N: usize> DirectionReaderIterator<B, N> {
    fn new(reader: B) -> Self {
        Self {
            inner: reader,
            buffer: [0; N],
            acc: Some(Vec::new()),
            res_queue: VecDeque::new(),
        }
    }
}

impl<B: Read, const N: usize> Iterator for DirectionReaderIterator<B, N> {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        debug!("next");
        loop {
            debug!("res_queue: {:?}", self.res_queue);
            if !self.res_queue.is_empty() {
                return self.res_queue.pop_front();
            }

            // End of iterator.
            let acc = self.acc.as_mut()?;

            let mut bread = self.inner.read(&mut self.buffer).unwrap();
            debug!("buffer: {:?}", std::str::from_utf8(&self.buffer).unwrap());
            debug!("read: {:?}", bread);
            acc.extend(&self.buffer[..bread]);
            // Retry to be sure we reach the eof.
            if bread < N {
                debug!("retry");
                bread = self.inner.read(&mut self.buffer).unwrap();
                acc.extend(&self.buffer[..bread]);
                debug!("buffer: {:?}", std::str::from_utf8(&self.buffer).unwrap());
                debug!("read: {:?}", bread);
            }
            debug!("acc: {:?}", std::str::from_utf8(acc.as_slice()).unwrap());

            let input = acc.clone();
            let input_str = std::str::from_utf8(input.as_slice()).unwrap();

            if bread < N {
                match parse_command_end(input_str) {
                    Ok((rest, parsed)) => {
                        debug!("rest: {rest:?}, parsed: {parsed:?}");
                        let iter = std::iter::repeat(parsed.0).take(parsed.1);
                        self.res_queue.extend(iter);
                        // End of iter.
                        self.acc = None;
                        continue;
                    }
                    Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                        println!("parse error {:?}", convert_error(input_str, e));
                        panic!()
                    }
                    _ => unreachable!(),
                }
            }

            match parse_commands(input_str) {
                Ok((rest, parsed)) => {
                    debug!("rest: {rest:?}, parsed: {parsed:?}");
                    for (dir, n) in parsed {
                        let iter = std::iter::repeat(dir).take(n);
                        self.res_queue.extend(iter);
                    }

                    acc.clear();
                    acc.extend(rest.as_bytes());
                    continue;
                }
                Err(Err::Incomplete(_)) => {
                    debug!("incomplete");
                    continue;
                }
                Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                    println!("parse error {:?}", convert_error(input_str, e));
                    panic!()
                }
            };
        }
    }
}

pub fn solve_part1_complex(input: impl Read) -> usize {
    let iter = DirectionReaderIterator::<_, 5>::new(input);
    let (_, tail_positions) = iter.fold(
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
