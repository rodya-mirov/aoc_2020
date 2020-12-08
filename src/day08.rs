use nom::lib::std::collections::HashSet;

const INPUT: &str = include_str!("input/8.txt");

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Op {
    Nop(isize),
    Acc(i64),
    Jmp(isize),
}

mod parse {

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char as exact_char, newline},
        combinator::{eof, map},
        multi::separated_list1,
        sequence::tuple,
        IResult,
    };

    use crate::lib::parse_usize;

    use super::Op;

    fn parse_acc(input: &str) -> IResult<&str, i64> {
        alt((
            map(tuple((exact_char('+'), parse_usize)), |(_, d)| d as i64),
            map(tuple((exact_char('-'), parse_usize)), |(_, d)| -(d as i64)),
        ))(input)
    }

    fn parse_op(input: &str) -> IResult<&str, Op> {
        alt((
            map(
                tuple((tag("nop"), exact_char(' '), parse_acc)),
                |(_, _, d)| Op::Nop(d as isize),
            ),
            map(
                tuple((tag("acc"), exact_char(' '), parse_acc)),
                |(_, _, d)| Op::Acc(d),
            ),
            map(
                tuple((tag("jmp"), exact_char(' '), parse_acc)),
                |(_, _, d)| Op::Jmp(d as isize),
            ),
        ))(input)
    }

    pub(super) fn parse(input: &str) -> Vec<Op> {
        let (_, ops) =
            map(tuple((separated_list1(newline, parse_op), eof)), |(v, _)| v)(input).unwrap();

        ops
    }
}

fn run_8a_with_input(input: &str) -> i64 {
    let ops = parse::parse(input);

    let mut acc = 0;
    let mut ip = 0;
    let mut seen = HashSet::new();

    loop {
        if seen.contains(&ip) {
            return acc;
        }
        seen.insert(ip);

        let op = ops[ip as usize];
        match op {
            Op::Acc(i) => {
                acc += i;
            }
            Op::Nop(_) => {
                // nothing
            }
            Op::Jmp(amt) => {
                ip += amt - 1;
            }
        }
        ip += 1;
    }
}

pub fn run_8a() -> i64 {
    run_8a_with_input(INPUT)
}

fn run_8b_with_input(input: &str) -> i64 {
    fn terminates(ops: &[Op]) -> Option<i64> {
        let mut acc = 0;
        let mut ip = 0;
        let mut seen = HashSet::new();

        let ilen = ops.len() as isize;

        while ip >= 0 && ip < ilen {
            if seen.contains(&ip) {
                return None;
            }
            seen.insert(ip);

            let op = ops[ip as usize];
            match op {
                Op::Acc(i) => {
                    acc += i;
                }
                Op::Nop(_) => {
                    // nothing
                }
                Op::Jmp(amt) => {
                    ip += amt - 1;
                }
            }
            ip += 1;
        }

        Some(acc)
    }

    let mut ops = parse::parse(input);

    for i in 0..ops.len() {
        let op = ops[i];
        match op {
            Op::Acc(_) => {}
            Op::Nop(amt) => {
                ops[i] = Op::Jmp(amt);
                if let Some(v) = terminates(&ops) {
                    return v;
                }
                ops[i] = Op::Nop(amt);
            }
            Op::Jmp(amt) => {
                ops[i] = Op::Nop(amt);
                if let Some(v) = terminates(&ops) {
                    return v;
                }
                ops[i] = Op::Jmp(amt);
            }
        }
    }

    panic!("No valid change found :(");
}

pub fn run_8b() -> i64 {
    run_8b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn sample_8a() {
        assert_eq!(run_8a_with_input(SAMPLE), 5);
    }

    #[test]
    fn sample_8b() {
        assert_eq!(run_8b_with_input(SAMPLE), 8);
    }
}
