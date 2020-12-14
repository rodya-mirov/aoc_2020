use nom::{
    branch::alt,
    character::complete::{char as exact_char, digit1, newline},
    combinator::{eof, map, opt},
    multi::separated_list1,
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

pub fn parse_lines<T, F: FnMut(&str) -> IResult<&str, T>>(
    f: F,
    input: &str,
) -> IResult<&str, Vec<T>> {
    let (input, out) = separated_list1(newline, f)(input)?;
    let (input, _) = pair(opt(newline), eof)(input)?;
    Ok((input, out))
}
