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

// CID ommitted
const REQUIRED_KEYS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

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
        use Passport::*;
        let passport = match valid {
            true => Valid(map),
            false => Invalid(map),
        };
        match passport {
            Passport::Valid(m) => {
                let mut items: Vec<(&String, &String)> = m.iter().collect();
                items.sort();
                for (key, value) in items.iter() {
                    match key.as_str() {
                        "byr" => {
                            if value.chars().count() == 4 {
                                let year: u32 = value.parse().unwrap();
                                if year < 1920 || year > 2002 {
                                    return Passport::Invalid(m);
                                }
                            } else {
                                return Passport::Invalid(m);
                            }
                        }
                        "iyr" => {
                            if value.chars().count() == 4 {
                                let year: u32 = value.parse().unwrap();
                                if year < 2010 || year > 2020 {
                                    return Passport::Invalid(m);
                                }
                            } else {
                                return Passport::Invalid(m);
                            }
                        }
                        "eyr" => {
                            if value.chars().count() == 4 {
                                let year: u32 = value.parse().unwrap();
                                if year < 2020 || year > 2030 {
                                    return Passport::Invalid(m);
                                }
                            } else {
                                return Passport::Invalid(m);
                            }
                        }
                        "hgt" => {
                            if value.ends_with("cm") {
                                let mut height: String = value.clone().to_string();
                                height.pop();
                                height.pop();
                                let height: u32 = height.parse().unwrap();
                                if height > 193 || height < 150 {
                                    return Passport::Invalid(m);
                                }
                            } else if value.ends_with("in") {
                                let mut height: String = value.clone().to_string();
                                height.pop();
                                height.pop();
                                let height: u32 = height.parse().unwrap();
                                if height > 76 || height < 59 {
                                    return Passport::Invalid(m);
                                }
                            } else {
                                return Passport::Invalid(m);
                            }
                        }
                        "hcl" => {
                            let mut color = value.chars();
                            let valid_digits = "0123456789abcdef";
                            if color.next() == Some('#') {
                                if color.clone().any(|c| !valid_digits.contains(c)) {
                                    return Passport::Invalid(m);
                                }
                                if color.count() != 6 {
                                    return Passport::Invalid(m);
                                }
                            } else {
                                return Passport::Invalid(m);
                            }
                        }
                        "ecl" => {
                            let valid_colors = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                            if !valid_colors.contains(&value.as_str()) {
                                return Passport::Invalid(m);
                            }
                        }
                        "pid" => {
                            if value.chars().count() != 9 {
                                return Passport::Invalid(m);
                            }
                            if value.parse::<u32>().is_err() {
                                return Passport::Invalid(m);
                            }
                        }
                        "cid" => (),
                        _ => return Passport::Invalid(m),
                    }
                }
                Passport::Valid(m)
            }
            Passport::Invalid(m) => Passport::Invalid(m),
        }
    }
}

trait Mode {
    fn create_passport(map: HashMap<String, String>) -> Passport;
}

#[derive(Debug)]
struct Passports<M> {
    inner: Vec<Passport>,
    // current: HashMap<String, String>,
    _mode: PhantomData<M>,
}

impl<'a, M: Mode> FromIterator<&'a String> for Passports<M> {
    fn from_iter<I: IntoIterator<Item = &'a String>>(iter: I) -> Self {
        let mut inner = vec![];
        let mut current: HashMap<String, String> = HashMap::new();
        for line in iter {
            if line.is_empty() {
                // println!("{:?}: {:?}", current.get("pid"), current.keys());
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
            .filter_map(|p| match p {
                Passport::Valid(v) => Some(v),
                _ => None,
            })
            .count()
    }
}
