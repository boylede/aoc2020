use crate::PartResult;
use std::str::FromStr;

pub fn part1(lines: &Vec<String>) -> PartResult {
    let passwords: Vec<Password> = lines
        .iter()
        .map(|line| line.parse::<Password>().unwrap())
        .collect();

    let number = passwords
        .iter()
        .filter(|password| password.is_valid_sled())
        .count();
    Ok(number.to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    let passwords: Vec<Password> = lines
        .iter()
        .map(|line| line.parse::<Password>().unwrap())
        .collect();

    let number = passwords
        .iter()
        .filter(|password| password.is_valid_tobaggan())
        .count();
    Ok(number.to_string())
}

#[derive(Debug)]
struct Password {
    a: i32,
    b: i32,
    letter: char,
    password: String,
}

#[derive(Debug)]
enum PasswordError {
    ParseError,
}

impl FromStr for Password {
    type Err = PasswordError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let chars: &[char] = &[':', '-', ' '];
        let mut parts = input.split(chars);
        let a = parts
            .next()
            .ok_or(PasswordError::ParseError)?
            .parse::<i32>()
            .unwrap();
        let b = parts
            .next()
            .ok_or(PasswordError::ParseError)?
            .parse::<i32>()
            .unwrap();
        let letter = parts
            .next()
            .ok_or(PasswordError::ParseError)?
            .chars()
            .next()
            .unwrap();
        let _ = parts.next();
        let password = parts.next().ok_or(PasswordError::ParseError)?.to_string();
        Ok(Password {
            a,
            b,
            letter,
            password,
        })
    }
}

impl Password {
    fn is_valid_sled(&self) -> bool {
        let number = self
            .password
            .chars()
            .filter(|&pc| pc == self.letter)
            .count() as i32;
        number >= self.a && number <= self.b
    }
    fn is_valid_tobaggan(&self) -> bool {
        let a_matches = self.password.chars().nth(self.a as usize - 1).unwrap() == self.letter;
        let b_matches = self.password.chars().nth(self.b as usize - 1).unwrap() == self.letter;
        match (a_matches, b_matches) {
            (true, true) => false,
            (true, false) => true,
            (false, true) => true,
            (false, false) => false,
        }
    }
}
