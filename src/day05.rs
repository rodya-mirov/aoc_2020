const INPUT: &str = include_str!("input/5.txt");

#[inline(always)]
fn read_seat_id(line: &str) -> u32 {
    let mut seat = 0;

    let mut chars = line.chars();

    for _ in 0..7 {
        let c = chars.next().unwrap();
        seat = (seat << 1) + (if c == 'B' { 1 } else { 0 });
    }

    for _ in 0..3 {
        let c = chars.next().unwrap();
        seat = (seat << 1) + (if c == 'R' { 1 } else { 0 });
    }

    assert!(chars.next().is_none());

    seat
}

fn run_5a_with_input(input: &str) -> u32 {
    input.lines().map(|line| read_seat_id(line)).max().unwrap()
}

pub fn run_5a() -> u32 {
    run_5a_with_input(INPUT)
}

fn run_5b_with_input(input: &str) -> u32 {
    let mut seats = input
        .lines()
        .map(|line| read_seat_id(line))
        .collect::<Vec<_>>();
    seats.sort();

    for i in 1..seats.len() {
        if seats[i - 1] + 2 == seats[i] {
            return seats[i] - 1;
        }
    }

    panic!("Seat not found, probably an input error");
}

pub fn run_5b() -> u32 {
    run_5b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seat_parse_tests() {
        assert_eq!(read_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(read_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(read_seat_id("BBFFBBFRLL"), 820);
    }
}
