use crate::PartResult;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::marker::PhantomData;

pub fn part1(lines: &Vec<String>) -> PartResult {
    let passports: Passports<Careless> = lines.iter().collect();
    Ok(passports.count_valid().to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    let passports: Passports<Strict> = lines.iter().collect();
    Ok(passports.count_valid().to_string())
}

#[derive(Debug)]
enum Passport {
    Valid(HashMap<String, String>),
    Invalid(HashMap<String, String>),
}

// cid ommitted
const REQUIRED_KEYS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const VALID_HEX_DIGITS: &str = "0123456789abcdef";
const VALID_EYE_COLORS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

trait Mode {
    fn create_passport(map: HashMap<String, String>) -> Passport;
}

struct Careless;
impl Mode for Careless {
    fn create_passport(map: HashMap<String, String>) -> Passport {
        let valid = REQUIRED_KEYS
            .iter()
            .all(|key| map.contains_key(key.to_owned()));
        use Passport::*;
        match valid {
            true => Valid(map),
            false => Invalid(map),
        }
    }
}

struct Strict;
impl Mode for Strict {
    fn create_passport(map: HashMap<String, String>) -> Passport {
        let valid = REQUIRED_KEYS
            .iter()
            .all(|key| map.contains_key(key.to_owned()));
        if valid {
            let mut items: Vec<(&String, &String)> = map.iter().collect();
            items.sort();
            for (key, value) in items.iter() {
                let strict_valid = match key.as_str() {
                    "byr" => valid_birth_year(value),
                    "iyr" => valid_issue_year(value),
                    "eyr" => valid_expiration_year(value),
                    "hgt" => valid_height(value),
                    "ecl" => valid_eye_color(value),
                    "hcl" => valid_hair_color(value),
                    "pid" => valid_pid(value),
                    "cid" => true,
                    _ => false,
                };
                if !strict_valid {
                    return Passport::Invalid(map);
                }
            }
            return Passport::Valid(map);
        } else {
            return Passport::Invalid(map);
        }
    }
}

fn is_between(value: &str, min: u32, max: u32) -> bool {
    let number: Result<u32, _> = value.parse();
    match number {
        Ok(num) => num >= min && num <= max,
        Err(_) => false,
    }
}

fn valid_birth_year(value: &str) -> bool {
    if value.chars().count() == 4 {
        is_between(value, 1920, 2002)
    } else {
        false
    }
}

fn valid_issue_year(value: &str) -> bool {
    if value.chars().count() == 4 {
        is_between(value, 2010, 2020)
    } else {
        false
    }
}

fn valid_expiration_year(value: &str) -> bool {
    if value.chars().count() == 4 {
        is_between(value, 2020, 2030)
    } else {
        false
    }
}

fn valid_height(value: &str) -> bool {
    let len = value.len();
    if value.ends_with("cm") {
        is_between(&value[0..len-2], 150, 193)
    } else if value.ends_with("in") {
        is_between(&value[0..len-2], 59, 76)
    } else {
        false
    }
}

fn valid_eye_color(value: &str) -> bool {
    if !VALID_EYE_COLORS.contains(&value) {
        false
    } else {
        true
    }
}

fn valid_hair_color(value: &str) -> bool {
    let length = value.len();
    let mut color = value.chars();
    if color.next() == Some('#') {
        if color.any(|c| !VALID_HEX_DIGITS.contains(c)) {
            false
        } else if length != 7 { // 6 required digits plus #
            false
        } else {
            true
        }
    } else {
        false
    }
}

fn valid_pid(value: &str) -> bool {
    if value.len() != 9 {
        false
    } else if value.parse::<u32>().is_err() {
        false
    } else {
        true
    }
}

#[derive(Debug)]
struct Passports<M> {
    inner: Vec<Passport>,
    _mode: PhantomData<M>,
}

impl<'a, M: Mode> FromIterator<&'a String> for Passports<M> {
    fn from_iter<I: IntoIterator<Item = &'a String>>(iter: I) -> Self {
        let mut inner = vec![];
        let mut current: HashMap<String, String> = HashMap::new();
        for line in iter {
            if line.is_empty() {
                let passport = M::create_passport(current.clone());
                inner.push(passport);
                current.clear();
            } else {
                let pairs = line.split(" ");
                for pair in pairs {
                    let mut p = pair.split(":");
                    let key = p.next().unwrap().to_string();
                    let value = p.next().unwrap().to_string();
                    current.insert(key, value);
                }
            }
        }
        if current.keys().count() > 0 {
            let passport = M::create_passport(current.clone());
            inner.push(passport);
            current.clear();
        }
        Passports {
            inner,
            _mode: <PhantomData<M> as Default>::default(),
        }
    }
}

impl<M> Passports<M> {
    fn count_valid(&self) -> usize {
        self.inner
            .iter()
            .filter(|p| match p {
                Passport::Valid(_) => true,
                _ => false,
            })
            .count()
    }
}
