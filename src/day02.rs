const INPUT: &str = include_str!("input/2.txt");

#[derive(Debug, Eq, PartialEq, Clone)]
struct PasswordLine {
    policy: Policy,
    password: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

mod parse {
    use super::{PasswordLine, Policy};

    use nom::character::complete::{alpha1, anychar, char};
    use nom::{
        bytes::complete::tag, character::complete::digit1, combinator::eof, combinator::map,
        sequence::tuple, IResult,
    };

    fn parse_num(input: &str) -> IResult<&str, usize> {
        map(digit1, |token: &str| token.parse::<usize>().unwrap())(input)
    }

    fn parse_line(input: &str) -> IResult<&str, PasswordLine> {
        let base_parser = tuple((
            parse_num,
            char('-'),
            parse_num,
            char(' '),
            anychar,
            tag(": "),
            alpha1,
            eof,
        ));

        let mut mapped = map(base_parser, |(min, _, max, _, letter, _, passsword, _)| {
            PasswordLine {
                policy: Policy { min, max, letter },
                password: passsword.to_string(),
            }
        });

        mapped(input)
    }

    pub(super) fn parse(input: &str) -> Vec<PasswordLine> {
        let mut out = Vec::new();
        for line in input.lines() {
            let (_, pwline) = parse_line(line).unwrap();
            out.push(pwline);
        }
        out
    }

    #[cfg(test)]
    mod parser_tests {
        use super::*;

        #[test]
        fn sample_parse() {
            assert_eq!(
                parse("1-3 a: abcde"),
                vec![PasswordLine {
                    password: "abcde".to_string(),
                    policy: Policy {
                        min: 1,
                        max: 3,
                        letter: 'a'
                    }
                }]
            );
            assert_eq!(
                parse("1-3 a: abcde"),
                vec![PasswordLine {
                    password: "abcde".to_string(),
                    policy: Policy {
                        min: 1,
                        max: 3,
                        letter: 'a'
                    }
                }]
            );
            assert_eq!(
                parse("1-3 b: cdefg"),
                vec![PasswordLine {
                    password: "cdefg".to_string(),
                    policy: Policy {
                        min: 1,
                        max: 3,
                        letter: 'b'
                    }
                }]
            );
            assert_eq!(
                parse("2-9 c: ccccccccc"),
                vec![PasswordLine {
                    password: "ccccccccc".to_string(),
                    policy: Policy {
                        min: 2,
                        max: 9,
                        letter: 'c'
                    }
                }]
            );
        }
    }
}

fn run_2a_with_input(input: &str) -> usize {
    let lines = parse::parse(input);

    let mut good = 0;
    for line in lines {
        let count = line
            .password
            .chars()
            .filter(|&c| c == line.policy.letter)
            .count();
        if count >= line.policy.min && count <= line.policy.max {
            good += 1;
        }
    }

    good
}

pub fn run_2a() -> usize {
    run_2a_with_input(INPUT)
}

fn run_2b_with_input(input: &str) -> usize {
    let lines = parse::parse(input);

    let mut good = 0;
    for line in lines {
        let fits = line
            .password
            .chars()
            .enumerate()
            // restricted just to the characters at the specified indices ...
            .filter(|(i, _c)| i + 1 == line.policy.min || i + 1 == line.policy.max)
            // count the ones that match the letter in the policy
            .filter(|(_i, c)| *c == line.policy.letter)
            .count();

        if fits == 1 {
            good += 1;
        }
    }

    good
}

pub fn run_2b() -> usize {
    run_2b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn sample_2a() {
        assert_eq!(run_2a_with_input(SAMPLE_INPUT), 2);
    }

    #[test]
    fn sample_2b() {
        assert_eq!(run_2b_with_input(SAMPLE_INPUT), 1);
    }
}
