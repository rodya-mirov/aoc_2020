const INPUT: &str = include_str!("input/1.txt");

fn run_1a_with_input(input: &str) -> i32 {
    let nums: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    for i in 1..nums.len() {
        for j in 0..i {
            if nums[i] + nums[j] == 2020 {
                return nums[i] * nums[j];
            }
        }
    }

    unreachable!("Puzzle input guaranteed to be valid")
}

pub fn run_1a() -> i32 {
    run_1a_with_input(INPUT)
}

fn run_1b_with_input(input: &str) -> i32 {
    let nums: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    for i in 2..nums.len() {
        for j in 1..i {
            for k in 0..j {
                if nums[i] + nums[j] + nums[k] == 2020 {
                    return nums[i] * nums[j] * nums[k];
                }
            }
        }
    }

    unreachable!("Puzzle input guaranteed to be valid")
}

pub fn run_1b() -> i32 {
    run_1b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1a() {
        let input = "1721
979
366
299
675
1456";

        let expected = 514579;
        let actual = run_1a_with_input(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn sample_1b() {
        let input = "1721
979
366
299
675
1456";

        let expected = 241861950;
        let actual = run_1b_with_input(input);

        assert_eq!(expected, actual);
    }
}
