use crate::{PartError, PartResult};
use std::collections::HashMap;
use std::iter::FromIterator;


pub fn part1(lines: &Vec<String>) -> PartResult {
    let passports: Passports = lines.iter().collect();
    Ok(passports.count_valid().to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    let passports: StrictPassports = lines.iter().collect();
    println!("found {} passports", passports.inner.len());
    Ok(passports.count_valid().to_string())
}

#[derive(Debug)]
enum Passport {
    Valid(HashMap<String, String>),
    Invalid(HashMap<String, String>),
}

// CID ommitted
const REQUIRED_KEYS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

impl Passport {
    fn careless_mode(map: HashMap<String, String>) -> Self {
        let valid = REQUIRED_KEYS
            .iter()
            .all(|key| map.contains_key(key.to_owned()));
        use Passport::*;
        match valid {
            true => Valid(map),
            false => Invalid(map),
        }
    }
    fn strict_mode(map: HashMap<String, String>) -> Self {
        let passport = Passport::careless_mode(map);
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
                                    println!("rejecting {} as birth year 1920-2002", value);
                                    return Passport::Invalid(m)
                                }
                            } else {
                                println!("rejecting {} as birth year", value);
                                return Passport::Invalid(m)
                            }
                        },
                        "iyr" => {
                            if value.chars().count() == 4 {
                                let year: u32 = value.parse().unwrap();
                                if year < 2010 || year > 2020 {
                                    println!("rejecting {} as issue year 2010-2020", value);
                                    return Passport::Invalid(m)
                                }
                            } else {
                                println!("rejecting {} as issue year", value);
                                return Passport::Invalid(m)
                            }
                        },
                        "eyr" => {
                            if value.chars().count() == 4 {
                                let year: u32 = value.parse().unwrap();
                                if year < 2020 || year > 2030 {
                                    println!("rejecting {} as expire year 2020-2030", value);
                                    return Passport::Invalid(m)
                                }
                            } else {
                                println!("rejecting {} as expire year", value);
                                return Passport::Invalid(m)
                            }
                        },
                        "hgt" => {
                            if value.ends_with("cm") {
                                let mut height: String = value.clone().to_string();
                                height.pop();
                                height.pop();
                                let height: u32 = height.parse().unwrap();
                                if height > 193 || height < 150 {
                                    println!("rejecting {} as height", value);
                                    return Passport::Invalid(m)
                                }
                                
                            } else if value.ends_with("in") {
                                let mut height: String = value.clone().to_string();
                                height.pop();
                                height.pop();
                                let height: u32 = height.parse().unwrap();
                                if height > 76 || height < 59 {
                                    println!("rejecting {} as height", value);
                                    return Passport::Invalid(m)
                                }
                            } else {
                                println!("rejecting {} as height", value);
                                return Passport::Invalid(m)
                            }
                        },
                        "hcl" => {
                            let mut color = value.chars();
                            let valid_digits = "0123456789abcdef";
                            if color.next() == Some('#') {
                                if color.clone().any(|c| !valid_digits.contains(c)) {
                                    println!("rejecting {} as hair color because of invalid digit", value);
                                    return Passport::Invalid(m);
                                }
                                if color.count() != 6 {
                                    println!("rejecting {} as hair color because of length", value);
                                    return Passport::Invalid(m);
                                }
                            } else {
                                println!("rejecting {} as hair color because lacks #", value);
                                return Passport::Invalid(m);
                            }
                        },
                        "ecl" => {
                            let valid_colors = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                            if !valid_colors.contains(&value.as_str()) {
                                println!("rejecting {} as eye color", value);
                                return Passport::Invalid(m)
                            }
                        },
                        "pid" => {
                            if value.chars().count() != 9 {
                                println!("rejecting {} as pid", value);
                                return Passport::Invalid(m);   
                            }
                            if value.parse::<u32>().is_err() {
                                println!("rejecting {} as pid", value);
                                return Passport::Invalid(m);   
                            }
                        },
                        "cid" => (),
                        _ => return {
                            println!("contained an invalid key {}", key);
                            Passport::Invalid(m)
                        },
                    }
                }
                Passport::Valid(m)
            }
            Passport::Invalid(m) => {
                println!("lacked required keys");
                Passport::Invalid(m)
            },
        }
        
    }
}


#[derive(Debug)]
struct StrictPassports {
    inner: Vec<Passport>,
    // current: HashMap<String, String>,
}

impl<'a> FromIterator<&'a String> for StrictPassports {
    fn from_iter<I: IntoIterator<Item = &'a String>>(iter: I) -> Self {
        let mut inner = vec![];
        let mut current: HashMap<String, String> = HashMap::new();
        for line in iter {
            if line.is_empty() {
                // println!("{:?}: {:?}", current.get("pid"), current.keys());
                let passport = Passport::strict_mode(current.clone());
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
            let passport = Passport::strict_mode(current.clone());
                inner.push(passport);
                current.clear();
        }
        StrictPassports { inner }
    }
}


impl StrictPassports {
    fn count_valid(&self) -> usize {
        self.inner
            .iter()
            .inspect(|p| {
                match p {
                    Passport::Valid(_) => (),
                    Passport::Invalid(p) => why_invalid(p),
                }
            })
            .filter_map(|p| match p {
                Passport::Valid(v) => Some(v),
                _ => None,
            })
            // .map(|p| {
            //     let mut k: Vec<&String> = p.keys().collect();
            //     k.sort();
            //     k
            // })
            // .inspect(|p| println!("{:?}", p))
            .count()
    }
}

#[derive(Debug)]
struct Passports {
    inner: Vec<Passport>,
    // current: HashMap<String, String>,
}

impl<'a> FromIterator<&'a String> for Passports {
    fn from_iter<I: IntoIterator<Item = &'a String>>(iter: I) -> Self {
        let mut inner = vec![];
        let mut current: HashMap<String, String> = HashMap::new();
        for line in iter {
            if line.is_empty() {
                // println!("{:?}: {:?}", current.get("pid"), current.keys());
                let passport = Passport::careless_mode(current.clone());
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
            let passport = Passport::careless_mode(current.clone());
                inner.push(passport);
                current.clear();
        }
        Passports { inner }
    }
}

impl Passports {
    fn count_valid(&self) -> usize {
        self.inner
            .iter()
            .inspect(|p| {
                match p {
                    Passport::Valid(_) => (),
                    Passport::Invalid(p) => why_invalid(p),
                }
            })
            .filter_map(|p| match p {
                Passport::Valid(v) => Some(v),
                _ => None,
            })
            // .map(|p| {
            //     let mut k: Vec<&String> = p.keys().collect();
            //     k.sort();
            //     k
            // })
            // .inspect(|p| println!("{:?}", p))
            .count()
    }
}


fn why_invalid(map: &HashMap<String, String>) {
    let mut missing_keys = vec![];
    for key in REQUIRED_KEYS.iter() {
        if !map.contains_key(&key.to_string()) {
            missing_keys.push(key);
        }
    }
    println!("missing {:?}", missing_keys);
}