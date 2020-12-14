const INPUT: &str = "1002394
13,x,x,41,x,x,x,37,x,x,x,x,x,419,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,19,x,x,x,23,x,x,x,x,x,29,x,421,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17";

fn parse_a(input: &str) -> (usize, Vec<usize>) {
    let mut lines = input.lines();

    let id = lines.next().unwrap().parse::<usize>().unwrap();

    let bus_ids = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|&s| s != "x")
        .map(|d| d.parse::<usize>().unwrap())
        .collect();

    assert_eq!(lines.next(), None);

    (id, bus_ids)
}

fn run_13a_with_input(input: &str) -> usize {
    let (start_time, bus_ids) = parse_a(input);

    let mut least_wait = usize::max_value();
    let mut running_score = 0;

    for bus_id in bus_ids {
        let wait = bus_id - (start_time % bus_id);
        if wait < least_wait {
            least_wait = wait;
            running_score = least_wait * bus_id;
        }
    }

    assert_ne!(running_score, 0);

    running_score
}

pub fn run_13a() -> usize {
    run_13a_with_input(INPUT)
}

fn parse_b(input: &str) -> Vec<Option<usize>> {
    let mut lines = input.lines();

    lines.next().unwrap(); // skip

    let out = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| {
            if s == "x" {
                None
            } else {
                let val = s.parse().unwrap();
                Some(val)
            }
        })
        .collect();

    assert!(lines.next().is_none());

    out
}

fn run_13b_with_input(input: &str) -> usize {
    let constraints = parse_b(input);

    // assume: all bus ids are primes; not checked
    let mut prime_prods: usize = 1;
    let mut running_index: usize = 0;
    for (i, p) in constraints.into_iter().enumerate() {
        if p.is_none() {
            continue;
        }

        let p = p.unwrap();
        let desired_mod = (p - (i % p)) % p;

        while running_index % p != desired_mod {
            // this doesn't alter the correct mods previously acquired
            running_index += prime_prods;
        }

        prime_prods *= p;
    }

    // Pretty sure this doesn't matter but why not try, right?
    running_index = running_index % prime_prods;

    running_index
}

pub fn run_13b() -> usize {
    run_13b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_A: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn sample_13a() {
        assert_eq!(run_13a_with_input(SAMPLE_A), 295);
    }

    #[test]
    fn sample_13b() {
        assert_eq!(run_13b_with_input(SAMPLE_A), 1068781);
        assert_eq!(run_13b_with_input("\n17,x,13,19"), 3417);
        assert_eq!(run_13b_with_input("\n67,7,59,61"), 754018);
        assert_eq!(run_13b_with_input("\n67,x,7,59,61"), 779210);
        assert_eq!(run_13b_with_input("\n67,7,x,59,61"), 1261476);
        assert_eq!(run_13b_with_input("\n1789,37,47,1889"), 1202161486);
    }
}
