use std::{cmp::Ordering, ops::Deref};

use itertools::{EitherOrBoth, Itertools};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace1},
    combinator::{into, map},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, terminated, tuple},
    IResult,
};

use crate::quick_sort;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct Uint(usize);
impl Uint {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(digit1, |x: &str| Self(x.parse::<usize>().expect("safe")))(i)
    }
}

impl Deref for Uint {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
#[repr(transparent)]
struct List(Box<[Expr]>);
impl List {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            delimited(tag("["), separated_list0(tag(","), Expr::parse), tag("]")),
            |v| List(v.into_boxed_slice()),
        )(i)
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for e in self.0.iter().zip_longest(other.0.iter()) {
            match e {
                EitherOrBoth::Right(_) => {
                    return Ordering::Less;
                }
                EitherOrBoth::Left(_) => {
                    return Ordering::Greater;
                }
                EitherOrBoth::Both(l, r) => {
                    let cmp = l.cmp(r);
                    match cmp {
                        Ordering::Less | Ordering::Greater => {
                            return cmp;
                        }
                        Ordering::Equal => {
                            continue;
                        }
                    }
                }
            };
        }
        Ordering::Equal
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for List {}

#[repr(transparent)]
pub struct Packets(Box<[List]>);

impl Packets {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        map(separated_list1(multispace1, List::parse), |v| {
            Self(v.into())
        })(i)
    }

    pub fn decoder_key(&mut self) -> usize {
        quick_sort::quick_sort(&mut self.0);
        let signal2 = List(vec![List(vec![Uint(2).into()].into()).into()].into());
        let signal6 = List(vec![List(vec![Uint(6).into()].into()).into()].into());
        let s2 = self.0.iter().position(|e| e == &signal2).unwrap() + 1;
        let s6 = self.0.iter().position(|e| e == &signal6).unwrap() + 1;

        s2 * s6
    }
}

#[derive(Debug)]
enum Expr {
    Uint(Uint),
    List(List),
}

impl Expr {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((into(Uint::parse), into(List::parse)))(i)
    }
}

impl Ord for Expr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Expr::Uint(l), Expr::Uint(r)) => l.cmp(&r),
            (Expr::List(l), Expr::List(r)) => l.cmp(&r),
            (Expr::List(l), Expr::Uint(r)) => {
                let r = List(vec![Expr::Uint(r.clone())].into_boxed_slice());
                l.cmp(&r)
            }
            (Expr::Uint(l), Expr::List(r)) => {
                let l = List(vec![Expr::Uint(l.clone())].into_boxed_slice());
                l.cmp(&r)
            }
        }
    }
}

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for Expr {}

impl From<Uint> for Expr {
    fn from(value: Uint) -> Self {
        Expr::Uint(value)
    }
}

impl From<List> for Expr {
    fn from(value: List) -> Self {
        Expr::List(value)
    }
}

#[derive(Debug)]
pub(crate) struct Pair {
    left: List,
    right: List,
}

impl Pair {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            tuple((terminated(List::parse, line_ending), List::parse)),
            |(left, right)| Self { left, right },
        )(i)
    }

    fn is_ordered(&self) -> bool {
        self.left.cmp(&self.right).is_le()
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub(crate) struct Pairs(Box<[Pair]>);

impl Pairs {
    pub(crate) fn parse(i: &str) -> IResult<&str, Self> {
        map(separated_list1(multispace1, Pair::parse), |v| {
            Pairs(v.into_boxed_slice())
        })(i)
    }

    pub(crate) fn ordered_pairs(&self) -> impl Iterator<Item = (usize, &Pair)> {
        self.0
            .iter()
            .enumerate()
            .filter(|(_i, p)| Pair::is_ordered(p))
    }
}
