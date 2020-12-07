use std::collections::HashMap;

const INPUT: &str = include_str!("input/7.txt");

mod parse {
    use std::collections::HashMap;

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, char as exact_char, newline},
        combinator::{eof, map},
        multi::separated_list1,
        sequence::{pair, tuple},
        IResult,
    };

    use crate::lib::parse_usize;

    struct Rule {
        name: String,
        children: HashMap<String, usize>,
    }

    fn parse_bag_descr(input: &str) -> IResult<&str, String> {
        map(
            tuple((alpha1, exact_char(' '), alpha1)),
            |(adj1, _, adj2): (&str, _, &str)| {
                let out = adj1.to_string() + " " + adj2;
                out
            },
        )(input)
    }

    fn parse_bag_or_bags(input: &str) -> IResult<&str, &str> {
        alt((tag("bags"), tag("bag")))(input)
    }

    fn parse_bag_contents(input: &str) -> IResult<&str, HashMap<String, usize>> {
        alt((
            map(tag("no other bags."), |_| HashMap::new()),
            map(
                pair(
                    separated_list1(
                        tag(", "),
                        tuple((
                            parse_usize,
                            exact_char(' '),
                            parse_bag_descr,
                            exact_char(' '),
                            parse_bag_or_bags,
                        )),
                    ),
                    exact_char('.'),
                ),
                |(list, _): (Vec<(usize, char, String, char, &str)>, char)| {
                    let mut out = HashMap::new();
                    for (num, _, kind, _, _) in list {
                        out.insert(kind, num);
                    }
                    out
                },
            ),
        ))(input)
    }

    fn parse_line(input: &str) -> IResult<&str, Rule> {
        map(
            tuple((parse_bag_descr, tag(" bags contain "), parse_bag_contents)),
            |(descr, _, contents): (String, _, HashMap<String, usize>)| Rule {
                name: descr,
                children: contents,
            },
        )(input)
    }

    fn parse_helper(input: &str) -> IResult<&str, Vec<Rule>> {
        let (input, rules) = separated_list1(newline, parse_line)(input)?;
        let (_, _) = eof(input)?;
        Ok(("", rules))
    }

    pub(super) fn parse(input: &str) -> HashMap<String, HashMap<String, usize>> {
        let (_, out) = parse_helper(input).unwrap();
        out.into_iter().fold(HashMap::new(), |mut acc, next| {
            acc.insert(next.name, next.children);
            acc
        })
    }
}

fn run_7a_with_input(input: &str) -> usize {
    let rules = parse::parse(input);

    let goal_bag = "shiny gold";

    // TODO perf: this would be much much faster with caching
    fn dfs(
        rules: &HashMap<String, HashMap<String, usize>>,
        goal_top: &str,
        goal_contents: &str,
    ) -> bool {
        if goal_top == goal_contents {
            return true;
        }

        let children = rules.get(goal_top).unwrap();
        if children.contains_key(goal_contents) {
            return true;
        } else {
            children.keys().any(|key| dfs(rules, key, goal_contents))
        }
    }

    rules
        .keys()
        .filter(|k| k.as_str() != goal_bag)
        .filter(|k| dfs(&rules, k, goal_bag))
        .count()
}

pub fn run_7a() -> usize {
    run_7a_with_input(INPUT)
}

fn run_7b_with_input(input: &str) -> usize {
    let rules = parse::parse(input);

    let goal_bag = "shiny gold";

    // TODO perf: this would be much much faster with caching
    fn dfs(rules: &HashMap<String, HashMap<String, usize>>, top_bag: &str) -> usize {
        1 + rules
            .get(top_bag)
            .unwrap()
            .iter()
            .map(|(kind, count)| count * dfs(rules, kind))
            .sum::<usize>()
    }

    // You don't count the top bag
    dfs(&rules, goal_bag) - 1
}

pub fn run_7b() -> usize {
    run_7b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn sample_7a() {
        assert_eq!(run_7a_with_input(SAMPLE_INPUT), 4);
    }

    #[test]
    fn sample_7b() {
        assert_eq!(run_7b_with_input(SAMPLE_INPUT), 32);
    }

    const SAMPLE_INPUT_2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn sample_7b_2() {
        assert_eq!(run_7b_with_input(SAMPLE_INPUT_2), 126);
    }
}
