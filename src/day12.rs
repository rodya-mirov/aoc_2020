const INPUT: &str = include_str!("input/12.txt");

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Move {
    Dir(Dir, i64),
    Turn(Turn),
    Forward(i64),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Dir {
    N,
    S,
    E,
    W,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Turn {
    L90,
    L180,
    L270,
    R90,
    R180,
    R270,
}

mod parse {
    use super::{Dir, Move, Turn};

    use crate::lib::{maybe_newline, parse_i64};

    use nom::{
        character::complete::{anychar, char as exact_char},
        combinator::eof,
        multi::separated_list1,
        sequence::tuple,
        IResult,
    };

    fn parse_cmd(input: &str) -> IResult<&str, Move> {
        let (input, next_char) = anychar(input)?;
        let (input, val) = parse_i64(input)?;
        match next_char {
            'N' => Ok((input, Move::Dir(Dir::N, val))),
            'S' => Ok((input, Move::Dir(Dir::S, val))),
            'E' => Ok((input, Move::Dir(Dir::E, val))),
            'W' => Ok((input, Move::Dir(Dir::W, val))),
            'F' => Ok((input, Move::Forward(val))),
            'L' => {
                match val {
                    90 => Ok((input, Move::Turn(Turn::L90))),
                    180 => Ok((input, Move::Turn(Turn::L180))),
                    270 => Ok((input, Move::Turn(Turn::L270))),
                    other => {
                        // TODO: make this a nom result somehow? idk
                        panic!("Unrecognized L turn amount {}", other)
                    }
                }
            }
            'R' => {
                match val {
                    90 => Ok((input, Move::Turn(Turn::R90))),
                    180 => Ok((input, Move::Turn(Turn::R180))),
                    270 => Ok((input, Move::Turn(Turn::R270))),
                    other => {
                        // TODO: make this a nom result somehow? idk
                        panic!("Unrecognized R turn amount {}", other)
                    }
                }
            }
            _ => {
                // TODO: make this a nom result somehow? idk
                panic!("Unrecognized direction tag {}", next_char);
            }
        }
    }

    fn parse_helper(input: &str) -> IResult<&str, Vec<Move>> {
        let (input, moves) = separated_list1(exact_char('\n'), parse_cmd)(input)?;
        let (input, _) = tuple((maybe_newline, eof))(input)?;

        Ok((input, moves))
    }

    pub(super) fn parse(input: &str) -> Vec<Move> {
        let (_empty, out) = parse_helper(input).unwrap();
        out
    }
}

const fn rotate_dir(dir: Dir, turn: Turn) -> Dir {
    #[inline(always)]
    const fn left_90(dir: Dir) -> Dir {
        match dir {
            Dir::E => Dir::N,
            Dir::N => Dir::W,
            Dir::W => Dir::S,
            Dir::S => Dir::E,
        }
    }

    // The following is all const and LLVM will optimize all the method calls away into
    // a single lookup table
    match turn {
        Turn::L90 => left_90(dir),
        Turn::L180 => left_90(left_90(dir)),
        Turn::L270 => left_90(left_90(left_90(dir))),
        Turn::R90 => left_90(left_90(left_90(dir))),
        Turn::R180 => left_90(left_90(dir)),
        Turn::R270 => left_90(dir),
    }
}

const fn rotate_pos(pos: Pos, turn: Turn) -> Pos {
    #[inline(always)]
    const fn left_90(pos: Pos) -> Pos {
        // right90: (10, -4) -> (4, 10)
        // left90: (4, 10) -> (10, -4)
        // (1, 0) -> (0, -1)
        // (0, -1) -> (-1, 0)
        // (-1, 0) -> (0, 1)
        // (0, 1) -> (1, 0)
        Pos {
            x: pos.y,
            y: -pos.x,
        }
    }

    // The following is all const and LLVM will optimize all the method calls away into
    // a single lookup table
    match turn {
        Turn::L90 => left_90(pos),
        Turn::L180 => left_90(left_90(pos)),
        Turn::L270 => left_90(left_90(left_90(pos))),
        Turn::R90 => left_90(left_90(left_90(pos))),
        Turn::R180 => left_90(left_90(pos)),
        Turn::R270 => left_90(pos),
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
struct Pos {
    x: i64,
    y: i64,
}

fn run_12a_with_input(input: &str) -> i64 {
    let moves = parse::parse(input);
    let mut dir = Dir::E;
    let mut pos = Pos::default();

    for m in moves {
        match m {
            Move::Forward(val) => match dir {
                Dir::E => pos.x += val,
                Dir::W => pos.x -= val,
                Dir::N => pos.y -= val,
                Dir::S => pos.y += val,
            },
            Move::Turn(turn) => {
                dir = rotate_dir(dir, turn);
            }
            Move::Dir(dir, val) => match dir {
                Dir::E => pos.x += val,
                Dir::W => pos.x -= val,
                Dir::N => pos.y -= val,
                Dir::S => pos.y += val,
            },
        }
    }

    pos.x.abs() + pos.y.abs()
}

pub fn run_12a() -> i64 {
    run_12a_with_input(INPUT)
}

fn run_12b_with_input(input: &str) -> i64 {
    let moves = parse::parse(input);

    let mut pos = Pos::default();
    let mut waypoint_pos = Pos { x: 10, y: -1 };

    for m in moves {
        match m {
            Move::Forward(val) => {
                for _ in 0..val {
                    pos.x += waypoint_pos.x;
                    pos.y += waypoint_pos.y;
                }
            }
            Move::Turn(turn) => {
                waypoint_pos = rotate_pos(waypoint_pos, turn);
            }
            Move::Dir(dir, val) => match dir {
                Dir::E => waypoint_pos.x += val,
                Dir::W => waypoint_pos.x -= val,
                Dir::N => waypoint_pos.y -= val,
                Dir::S => waypoint_pos.y += val,
            },
        }
    }

    pos.x.abs() + pos.y.abs()
}

pub fn run_12b() -> i64 {
    run_12b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_12A: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn sample_12a() {
        assert_eq!(run_12a_with_input(SAMPLE_12A), 25);
    }

    #[test]
    fn sample_12b() {
        assert_eq!(run_12b_with_input(SAMPLE_12A), 286);
    }
}
