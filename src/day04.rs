const INPUT: &str = include_str!("input/4.txt");

#[derive(Default, Debug)]
struct ParsedPassport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

#[derive(Copy, Clone, Debug)]
enum Height {
    Cm(usize),
    In(usize),
}

impl ParsedPassport {
    fn is_valid_a(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid_b(&self) -> Result<(), String> {
        self.byr
            .as_ref()
            .map(|byr| parse::match_year(byr))
            .flatten()
            .filter(|&byr| 1920 <= byr && byr <= 2002)
            .ok_or_else(|| format!("Invalid byr: {:?}", self.byr))?;

        self.iyr
            .as_ref()
            .map(|byr| parse::match_year(byr))
            .flatten()
            .filter(|&byr| 2010 <= byr && byr <= 2020)
            .ok_or_else(|| format!("Invalid iyr: {:?}", self.iyr))?;

        self.eyr
            .as_ref()
            .map(|byr| parse::match_year(byr))
            .flatten()
            .filter(|&byr| 2020 <= byr && byr <= 2030)
            .ok_or_else(|| format!("Invalid eyr: {:?}", self.eyr))?;

        self.hgt
            .as_ref()
            .map(|hgt| parse::match_hgt(hgt))
            .flatten()
            .filter(|&hgt| match hgt {
                Height::Cm(cm) => cm >= 150 && cm <= 193,
                Height::In(inches) => inches >= 59 && inches <= 76,
            })
            .ok_or_else(|| format!("Invalid hgt: {:?}", self.hgt))?;

        self.hcl
            .as_ref()
            .map(|hcl| parse::match_hair_color(hcl))
            .flatten()
            .ok_or_else(|| format!("Invalid hcl: {:?}", self.hcl))?;

        self.ecl
            .as_ref()
            .map(|ecl| parse::match_ecl(ecl))
            .flatten()
            .ok_or_else(|| format!("Invalid ecl: {:?}", self.ecl))?;

        self.pid
            .as_ref()
            .map(|pid| parse::match_pid(pid))
            .flatten()
            .ok_or_else(|| format!("Invalid pid: {:?}", self.pid))?;

        Ok(())
    }
}

mod parse {
    use super::{Height, ParsedPassport};

    use std::collections::HashMap;

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char as exact_char, digit1, hex_digit1, none_of},
        combinator::{eof, map, verify},
        multi::fold_many1,
        sequence::tuple,
        IResult,
    };

    fn res_to_opt<O, E>(r: Result<O, E>) -> Option<()> {
        match r {
            Ok(_) => Some(()),
            Err(_) => None,
        }
    }

    pub(super) fn match_hgt(input: &str) -> Option<Height> {
        let res: IResult<&str, Height> = map(
            tuple((
                alt((
                    map(tuple((digit1, tag("cm"))), |(d, _): (&str, _)| {
                        let cm = d.parse::<usize>().unwrap();
                        Height::Cm(cm)
                    }),
                    map(tuple((digit1, tag("in"))), |(d, _): (&str, _)| {
                        let inches = d.parse::<usize>().unwrap();
                        Height::In(inches)
                    }),
                )),
                eof,
            )),
            |(b, _)| b,
        )(input);

        res.ok().map(|(_, h)| h)
    }

    pub fn match_pid(input: &str) -> Option<()> {
        let res: IResult<&str, ()> =
            map(tuple((verify(digit1, |s: &str| s.len() == 9), eof)), |_| ())(input);

        res_to_opt(res)
    }

    pub fn match_year(input: &str) -> Option<usize> {
        let res: IResult<&str, usize> = map(
            tuple((verify(digit1, |v: &str| v.len() == 4), eof)),
            |(digits, _): (&str, _)| digits.parse::<usize>().unwrap(),
        )(input);

        match res {
            Ok(("", year)) => Some(year),
            _ => None,
        }
    }

    pub fn match_hair_color(input: &str) -> Option<()> {
        let res: IResult<&str, ()> = map(
            tuple((
                exact_char('#'),
                verify(hex_digit1, |s: &str| s.len() == 6),
                eof,
            )),
            |_| (),
        )(input);

        res_to_opt(res)
    }

    pub fn match_ecl(input: &str) -> Option<()> {
        //amb blu brn gry grn hzl oth
        let res: IResult<&str, ()> = map(
            alt((
                tag("amb"),
                tag("blu"),
                tag("brn"),
                tag("gry"),
                tag("grn"),
                tag("hzl"),
                tag("oth"),
            )),
            |_| (),
        )(input);

        res_to_opt(res)
    }

    fn parse_field(input: &str) -> IResult<&str, (String, String)> {
        map(
            tuple((
                fold_many1(none_of(":"), String::new(), |mut acc, c| {
                    acc.push(c);
                    acc
                }),
                exact_char(':'),
                fold_many1(none_of(":"), String::new(), |mut acc, c| {
                    acc.push(c);
                    acc
                }),
                eof,
            )),
            |(k, _, v, _)| (k, v),
        )(input)
    }

    fn parse_passport(input: &str) -> IResult<&str, ParsedPassport> {
        let mut fields: HashMap<String, String> = HashMap::new();

        for field_decl in input.split_whitespace() {
            let (_, (k, v)) = parse_field(field_decl)?;
            if fields.insert(k, v).is_some() {
                panic!("Field should not be double assigned");
            }
        }

        let mut passport = ParsedPassport::default();

        for (k, v) in fields {
            let field_ref = match k.as_str() {
                "byr" => &mut passport.byr,
                "iyr" => &mut passport.iyr,
                "eyr" => &mut passport.eyr,
                "hgt" => &mut passport.hgt,
                "hcl" => &mut passport.hcl,
                "ecl" => &mut passport.ecl,
                "pid" => &mut passport.pid,
                "cid" => &mut passport.cid,
                other => {
                    panic!("Unrecognized field {}", other);
                }
            };
            *field_ref = Some(v);
        }

        Ok(("", passport))
    }

    pub(super) fn parse_input(input: &str) -> Vec<ParsedPassport> {
        let mut out = Vec::new();

        for passport in input.split("\n\n") {
            let (_, passport) = parse_passport(passport).unwrap();
            out.push(passport);
        }

        out
    }
}

fn run_4a_with_input(input: &str) -> usize {
    let passports = parse::parse_input(input);

    passports.iter().filter(|p| p.is_valid_a()).count()
}

pub fn run_4a() -> usize {
    run_4a_with_input(INPUT)
}

fn run_4b_with_input(input: &str) -> usize {
    let passports = parse::parse_input(input);

    passports.iter().filter(|p| p.is_valid_b().is_ok()).count()
}

pub fn run_4b() -> usize {
    run_4b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn sample_4a() {
        assert_eq!(run_4a_with_input(SAMPLE), 2);
    }

    const ALL_BAD: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const ALL_GOOD: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn sample_4b() {
        let bad = parse::parse_input(ALL_BAD);
        for b in &bad {
            let r = b.is_valid_b();
            assert!(r.is_err(), "Should be invalid: {:?}", b);
        }

        let good = parse::parse_input(ALL_GOOD);
        for g in &good {
            let r = g.is_valid_b();
            assert!(
                r.is_ok(),
                "Should be valid: {:?}; rejection: {:?}",
                g,
                r.err().unwrap()
            );
        }

        assert_eq!(run_4b_with_input(ALL_BAD), 0);
        assert_eq!(run_4b_with_input(ALL_GOOD), 4);
    }
}
