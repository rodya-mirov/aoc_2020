use nom::{
    branch::alt,
    character::complete::{char as exact_char, digit1},
    combinator::{map, opt},
    sequence::pair,
    IResult,
};

pub fn parse_usize(input: &str) -> IResult<&str, usize> {
    map(digit1, |d: &str| d.parse::<usize>().unwrap())(input)
}

pub fn parse_i64(input: &str) -> IResult<&str, i64> {
    alt((
        map(digit1, |d: &str| d.parse::<i64>().unwrap()),
        map(pair(exact_char('-'), digit1), |(_, d): (_, &str)| {
            -(d.parse::<i64>().unwrap())
        }),
    ))(input)
}

pub fn maybe_newline(input: &str) -> IResult<&str, ()> {
    map(opt(exact_char('\n')), |_| ())(input)
}
