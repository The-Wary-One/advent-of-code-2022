use std::{collections::HashSet, io::BufRead};

use log::debug;
use nom::{
    character::complete::{one_of, space1, u8},
    combinator::map,
    error::{convert_error, VerboseError},
    sequence::separated_pair,
    Err, IResult,
};
use smallvec::{Array, SmallVec};

use crate::{Direction, Position};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static GLOBAL: dhat::Alloc = dhat::Alloc;

#[inline(always)]
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

#[inline(always)]
fn parse_command(input: &str) -> IResult<&str, (Direction, u8), VerboseError<&str>> {
    separated_pair(parse_direction, space1, u8)(input)
}

#[derive(Clone)]
struct DirectionReaderIterator<B: BufRead, const N: usize>
where
    [Direction; N]: Array<Item = Direction>,
{
    inner: B,
    buffer: String,
    res_stack: SmallVec<[Direction; N]>,
    finished: bool,
}

impl<B: BufRead, const N: usize> DirectionReaderIterator<B, N>
where
    [Direction; N]: Array<Item = Direction>,
{
    #[inline(always)]
    fn new(reader: B) -> Self {
        Self {
            inner: reader,
            buffer: String::new(),
            res_stack: SmallVec::new(),
            finished: false,
        }
    }
}

impl<B: BufRead, const N: usize> Iterator for DirectionReaderIterator<B, N>
where
    [Direction; N]: Array<Item = Direction>,
{
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        debug!("next");
        loop {
            if !self.res_stack.is_empty() || self.finished {
                return self.res_stack.pop();
            }

            let mut bread = self.inner.read_line(&mut self.buffer).unwrap();
            debug!("buffer: {:?}, read: {:?}", self.buffer, bread);
            // Retry to be sure we reach the eof.
            if bread == 0 {
                debug!("retry");
                bread = self.inner.read_line(&mut self.buffer).unwrap();
                debug!("buffer: {:?}, read: {:?}", self.buffer, bread);
                // End of iterator.
                if bread == 0 {
                    self.finished = true;
                    continue;
                }
            }

            match parse_command(self.buffer.as_str()) {
                Ok((rest, parsed)) => {
                    debug!("rest: {rest:?}, parsed: {parsed:?}");
                    let iter = std::iter::repeat(parsed.0).take(parsed.1.into());
                    self.res_stack.extend(iter);
                    self.buffer.clear();
                    continue;
                }
                Err(Err::Incomplete(_)) => {
                    panic!()
                }
                Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                    println!("parse error {}", convert_error(self.buffer.as_str(), e));
                    panic!()
                }
            };
        }
    }
}

#[inline(always)]
pub fn solve_part1_complex(input: impl BufRead) -> usize {
    let iter = DirectionReaderIterator::<_, 20>::new(input);
    let (_, tail_positions) = iter.fold(
        (
            (Position { x: 0, y: 0 }, Position { x: 0, y: 0 }),
            HashSet::with_capacity(5900),
        ),
        |((last_head_pos, last_tail_pos), mut tail_positions), direction| {
            let new_head_pos = last_head_pos.move_to(direction);
            let new_tail_pos = if !new_head_pos.is_adjacent(last_tail_pos) {
                tail_positions.insert(last_head_pos);
                last_head_pos
            } else {
                last_tail_pos
            };
            ((new_head_pos, new_tail_pos), tail_positions)
        },
    );
    tail_positions.len()
}
