use std::collections::HashMap;

const INPUT: &str = include_str!("input/10.txt");

fn run_10a_with_input(input: &str) -> u64 {
    let mut nums = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    nums.sort();

    let mut ones = 0;
    let mut threes = 1; // incl the phone voltage

    let mut last = 0;

    for n in nums {
        match n - last {
            3 => {
                threes += 1;
            }
            2 => {}
            1 => {
                ones += 1;
            }
            d => {
                panic!("Unsupported voltage expansion {}", d)
            }
        }
        last = n;
    }

    ones * threes
}

pub fn run_10a() -> u64 {
    run_10a_with_input(INPUT)
}

fn run_10b_with_input(input: &str) -> u64 {
    let mut nums = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    nums.sort();

    // Code below assumes no duplicate voltage
    for i in 1..nums.len() {
        if nums[i - 1] == nums[i] {
            panic!("Duplicate voltage {}", nums[i]);
        }
    }

    let phone_voltage = nums.get(nums.len() - 1).unwrap() + 3;

    // (prev_voltage, arr_offset) -> num_combos
    type Cache = HashMap<(u64, usize), u64>;
    let mut cache: Cache = HashMap::new();

    fn count_valid(
        prev_voltage: u64,
        running_offset: usize,
        goal_voltage: u64,
        adapters: &[u64],
        cache: &mut Cache,
    ) -> u64 {
        let key = (prev_voltage, running_offset);
        if let Some(cached) = cache.get(&key).copied() {
            return cached;
        }

        let val: u64 = {
            if adapters.is_empty() {
                if prev_voltage + 3 >= goal_voltage {
                    1
                } else {
                    0
                }
            } else {
                let next = adapters[0];
                // If we skipped too far, stop immediately
                if prev_voltage + 3 < next {
                    0
                } else {
                    let mut total = 0;

                    // either include the next one or don't
                    let next_slice = &adapters[1..];
                    total += count_valid(
                        prev_voltage,
                        running_offset + 1,
                        goal_voltage,
                        next_slice,
                        cache,
                    );
                    total += count_valid(next, running_offset + 1, goal_voltage, next_slice, cache);

                    total
                }
            }
        };

        cache.insert(key, val);
        val
    }

    count_valid(0, 0, phone_voltage, &nums, &mut cache)
}

pub fn run_10b() -> u64 {
    run_10b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const SAMPLE_INPUT_2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn sample_10a() {
        assert_eq!(run_10a_with_input(SAMPLE_INPUT_1), 7 * 5);
        assert_eq!(run_10a_with_input(SAMPLE_INPUT_2), 22 * 10);
    }
}
