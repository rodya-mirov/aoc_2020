const INPUT: &str = include_str!("input/6.txt");

const LETTER_OFFSET: u8 = 'a' as u8;

#[derive(Default, Eq, PartialEq, Debug, Copy, Clone)]
struct CustomsGroup {
    seen: [bool; 26],
}

impl CustomsGroup {
    fn intersect(&mut self, other: CustomsGroup) {
        for i in 0..self.seen.len() {
            self.seen[i] &= other.seen[i];
        }
    }
}

fn run_6a_with_input(input: &str) -> usize {
    let mut current_group = CustomsGroup::default();
    let mut total_count = 0;

    for line in input.lines() {
        if line.is_empty() {
            total_count += current_group.seen.iter().copied().filter(|&b| b).count();
            current_group = CustomsGroup::default();
        } else {
            for c in line.chars() {
                match c {
                    'a'..='z' => {
                        let index: usize = (c as u8 - LETTER_OFFSET) as usize;
                        current_group.seen[index] = true;
                    }
                    _ => {
                        panic!("Unsupported character '{}'", c);
                    }
                }
            }
        }
    }

    total_count += current_group.seen.iter().copied().filter(|&b| b).count();

    total_count
}

fn run_6b_with_input(input: &str) -> usize {
    let mut current_group: Option<CustomsGroup> = None;
    let mut total_count = 0;

    for line in input.lines() {
        if line.is_empty() {
            total_count += current_group
                .map(|cg| cg.seen.iter().copied().filter(|&b| b).count())
                .unwrap_or(0);
            current_group = None;
        } else {
            let mut person = CustomsGroup::default();
            for c in line.chars() {
                match c {
                    'a'..='z' => {
                        let index: usize = (c as u8 - LETTER_OFFSET) as usize;
                        person.seen[index] = true;
                    }
                    _ => {
                        panic!("Unsupported character '{}'", c);
                    }
                }
            }
            current_group = match current_group {
                None => Some(person),
                Some(mut existing) => {
                    existing.intersect(person);
                    Some(existing)
                }
            };
        }
    }

    total_count += current_group
        .map(|cg| cg.seen.iter().copied().filter(|&b| b).count())
        .unwrap_or(0);

    total_count
}

pub fn run_6a() -> usize {
    run_6a_with_input(INPUT)
}

pub fn run_6b() -> usize {
    run_6b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn sample_6a() {
        assert_eq!(run_6a_with_input(SAMPLE), 11);
    }

    #[test]
    fn sample_6b() {
        assert_eq!(run_6b_with_input(SAMPLE), 6);
    }
}
