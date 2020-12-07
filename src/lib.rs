use nom::{character::complete::digit1, combinator::map, IResult};

pub fn parse_usize(input: &str) -> IResult<&str, usize> {
    map(digit1, |d: &str| d.parse::<usize>().unwrap())(input)
}
