pub(super) use miette::GraphicalReportHandler;
pub(super) use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace1, one_of, space1},
    combinator::{map, opt, value},
    error::ParseError,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};
pub(super) use nom_locate::LocatedSpan;
pub(super) use nom_supreme::{
    error::{BaseErrorKind, ErrorTree, GenericErrorTree},
    final_parser::final_parser,
};

#[derive(thiserror::Error, Debug, miette::Diagnostic)]
#[error("bad input")]
pub(super) struct BadInput<'a> {
    #[source_code]
    src: &'a str,

    #[label("{kind}")]
    bad_bit: miette::SourceSpan,

    kind: BaseErrorKind<&'a str, Box<dyn std::error::Error + Send + Sync>>,
}

pub(super) struct MonkeyBusiness {
    pub(super) monkeys: Box<[Monkey]>,
}

pub(super) type Span<'a> = LocatedSpan<&'a str>;

pub(super) fn parse_monkeys_unsafe(i: &str) -> MonkeyBusiness {
    let input = Span::new(i);
    let monkeys_res: Result<_, ErrorTree<Span>> =
        final_parser(MonkeyBusiness::parse::<ErrorTree<Span>>)(input);
    match monkeys_res {
        Ok(monkeys) => monkeys,
        Err(e) => {
            match e {
                GenericErrorTree::Base { location, kind } => {
                    let offset = location.location_offset().into();
                    let err = BadInput {
                        src: i,
                        bad_bit: miette::SourceSpan::new(offset, 0.into()),
                        kind,
                    };
                    let mut s = String::new();
                    GraphicalReportHandler::new()
                        .render_report(&mut s, &err)
                        .unwrap();
                    println!("{s}");
                }
                GenericErrorTree::Stack { .. } => todo!("stack"),
                GenericErrorTree::Alt(_) => todo!("alt"),
            }
            panic!("error");
        }
    }
}

impl MonkeyBusiness {
    fn parse<'a, E: ParseError<Span<'a>>>(i: Span<'a>) -> IResult<Span<'a>, MonkeyBusiness, E> {
        map(
            separated_list1(
                multispace1,
                preceded(delimited(tag("Monkey "), digit1, tag(":\n")), Monkey::parse),
            ),
            |monkeys| Self {
                monkeys: monkeys.into_boxed_slice(),
            },
        )(i)
    }
}

pub(super) struct Monkey {
    pub(super) items: Vec<usize>,
    pub(super) operation: Expr,
    pub(super) divisible_by: usize,
    pub(super) target: usize,
    pub(super) fallback: usize,
}

impl Monkey {
    fn parse<'a, E: ParseError<Span<'a>>>(i: Span<'a>) -> IResult<Span<'a>, Monkey, E> {
        map(
            tuple((
                Self::parse_items,
                Self::parse_operation,
                Self::parse_divisible_by,
                Self::parse_target,
                Self::parse_fallback,
            )),
            |t| t.into(),
        )(i)
    }

    fn parse_items<'a, E: ParseError<Span<'a>>>(i: Span<'a>) -> IResult<Span<'a>, Vec<usize>, E> {
        map(
            delimited(
                tag("  Starting items: "),
                separated_list0(tag(", "), digit1),
                line_ending,
            ),
            |v: Vec<Span>| {
                v.iter()
                    .map(|s| s.parse::<usize>().expect("safe"))
                    .collect()
            },
        )(i)
    }

    fn parse_operation<'a, E: ParseError<Span<'a>>>(i: Span<'a>) -> IResult<Span<'a>, Expr, E> {
        delimited(tag("  Operation: "), Expr::parse, line_ending)(i)
    }

    fn parse_divisible_by<'a, E: ParseError<Span<'a>>>(i: Span<'a>) -> IResult<Span<'a>, usize, E> {
        map(
            delimited(tag("  Test: divisible by "), digit1, line_ending),
            |s: Span| s.parse::<usize>().expect("safe"),
        )(i)
    }

    fn parse_target<'a, E: ParseError<Span<'a>>>(i: Span<'a>) -> IResult<Span<'a>, usize, E> {
        map(
            delimited(tag("    If true: throw to monkey "), digit1, line_ending),
            |s: Span| s.parse::<usize>().expect("safe"),
        )(i)
    }

    fn parse_fallback<'a, E: ParseError<Span<'a>>>(i: Span<'a>) -> IResult<Span<'a>, usize, E> {
        map(
            delimited(
                tag("    If false: throw to monkey "),
                digit1,
                opt(line_ending),
            ),
            |s: Span| s.parse::<usize>().expect("safe"),
        )(i)
    }
}

impl From<(Vec<usize>, Expr, usize, usize, usize)> for Monkey {
    fn from(value: (Vec<usize>, Expr, usize, usize, usize)) -> Self {
        Self {
            items: value.0,
            operation: value.1,
            divisible_by: value.2,
            target: value.3,
            fallback: value.4,
        }
    }
}

#[derive(Clone)]
pub(super) enum Value {
    Constant(usize),
    Input,
}

pub(super) enum Op {
    Add,
    Mul,
}

pub(super) struct Expr {
    op: Op,
    a: Value,
    b: Value,
}

impl Expr {
    pub(super) fn exec(&self, input: &usize) -> usize {
        use Value::*;
        let a = match &self.a {
            Constant(c) => c,
            Input => input,
        };
        let b = match &self.b {
            Constant(c) => c,
            Input => input,
        };
        use Op::*;
        match self.op {
            Add => a + b,
            Mul => a * b,
        }
    }

    fn parse<'a, E: ParseError<Span<'a>>>(i: Span<'a>) -> IResult<Span<'a>, Expr, E> {
        map(
            preceded(
                tag("new = "),
                tuple((
                    alt((
                        value(Value::Input, tag("old")),
                        map(digit1, |n: Span| Value::Constant(n.parse().unwrap())),
                    )),
                    delimited(
                        space1,
                        map(one_of("+*"), |i| match i {
                            '+' => Op::Add,
                            '*' => Op::Mul,
                            _ => unreachable!(),
                        }),
                        space1,
                    ),
                    alt((
                        value(Value::Input, tag("old")),
                        map(digit1, |n: Span| Value::Constant(n.parse().unwrap())),
                    )),
                )),
            ),
            |(a, op, b)| Expr { op, a, b },
        )(i)
    }
}
