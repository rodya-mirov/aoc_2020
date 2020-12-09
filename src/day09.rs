const INPUT: &str = include_str!("input/9.txt");

fn run_9a_with_input(input: &str, cipher_len: usize) -> u32 {
    let mut cipher = Vec::with_capacity(cipher_len);
    let mut lines = input.lines();

    for _ in 0..cipher_len {
        cipher.push(lines.next().unwrap().parse().unwrap());
    }

    fn fits(cipher: &[u32], goal: u32) -> bool {
        for i in 1..cipher.len() {
            for j in 0..i {
                if cipher[i] + cipher[j] == goal {
                    return true;
                }
            }
        }
        false
    }

    let mut ctr = 0;
    for line in lines {
        let num: u32 = line.parse().unwrap();
        if !fits(&cipher, num) {
            return num;
        }

        cipher[ctr] = num;
        ctr = (ctr + 1) % cipher_len;
    }

    panic!("No error found")
}

fn run_9b_with_input(input: &str, cipher_len: usize) -> u32 {
    let mut cipher = Vec::with_capacity(cipher_len);
    let mut lines = input.lines().map(|n| n.parse::<u32>().unwrap());

    for _ in 0..cipher_len {
        cipher.push(lines.next().unwrap());
    }

    fn fits(cipher: &[u32], goal: u32) -> bool {
        for i in 1..cipher.len() {
            for j in 0..i {
                if cipher[i] + cipher[j] == goal {
                    return true;
                }
            }
        }
        false
    }

    fn find_exception(
        cipher: &mut Vec<u32>,
        nums: &mut impl Iterator<Item = u32>,
        cipher_len: usize,
    ) -> u32 {
        for num in nums {
            let len = cipher.len();
            if !fits(&cipher[len - cipher_len..len], num) {
                return num;
            }

            cipher.push(num);
        }

        panic!("No exception found")
    }

    let exception = find_exception(&mut cipher, &mut lines, cipher_len);

    let cipher_len = cipher.len();
    for start in 0..cipher_len {
        let mut total = cipher[start];

        let mut min = total;
        let mut max = total;

        let mut next_ind = start;

        while total < exception {
            next_ind += 1;
            let next = cipher[next_ind];
            min = min.min(next);
            max = max.max(next);
            total += next;
        }

        if total == exception {
            return min + max;
        }
    }

    panic!("No contiguous sum found")
}

pub fn run_9a() -> u32 {
    run_9a_with_input(INPUT, 25)
}

pub fn run_9b() -> u32 {
    run_9b_with_input(INPUT, 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_9A: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_sample_9a() {
        assert_eq!(run_9a_with_input(SAMPLE_9A, 5), 127);
    }

    #[test]
    fn test_sample_9n() {
        assert_eq!(run_9b_with_input(SAMPLE_9A, 5), 62);
    }
}
