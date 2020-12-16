use crate::PartResult;
use std::collections::HashSet;

pub fn part1(lines: &Vec<String>) -> PartResult {
    let program: Vec<(Operation, i32)> = lines
        .iter()
        .map(|line| {
            let mut instruction = line.split(" ");
            (
                get_operation(&mut instruction).unwrap(),
                get_argument(&mut instruction).unwrap(),
            )
        })
        .collect();
    let mut visited: HashSet<usize> = HashSet::new();
    let mut accumulator: i32 = 0;
    let mut current: usize = 0;
    let mut stop = false;
    while !stop {
        visited.insert(current);
        use Operation::*;
        match program[current] {
            (Acc, arg) => {
                accumulator += arg;
                current += 1;
            }
            (Jmp, arg) => current = ((current as i32) + arg) as usize,
            _ => current += 1,
        }
        if visited.contains(&current) {
            stop = true;
        }
    }

    Ok(accumulator.to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    let program: Vec<(Operation, i32)> = lines
        .iter()
        .map(|line| {
            let mut instruction = line.split(" ");
            (
                get_operation(&mut instruction).unwrap(),
                get_argument(&mut instruction).unwrap(),
            )
        })
        .collect();

    let result: i32 = program
        .iter()
        .enumerate()
        .filter_map(|(i, (op, _))| {
            if *op == Operation::Jmp || *op == Operation::Nop {
                Some(i)
            } else {
                None
            }
        })
        .filter_map(|i| test_run(program.clone(), i))
        .next()
        .unwrap();

    Ok(result.to_string())
}

fn test_run(mut program: Vec<(Operation, i32)>, instruction: usize) -> Option<i32> {
    let (op, _) = &mut program[instruction];
    use Operation::*;
    *op = match op {
        Jmp => Nop,
        Nop => Jmp,
        Acc => panic!("changed wrong instruction"),
    };
    let mut visited: HashSet<usize> = HashSet::new();
    let mut accumulator: i32 = 0;
    let mut current: usize = 0;
    loop {
        visited.insert(current);
        use Operation::*;
        match program[current] {
            (Acc, arg) => {
                accumulator += arg;
                current += 1;
            }
            (Jmp, arg) => current = ((current as i32) + arg) as usize,
            _ => current += 1,
        }
        if visited.contains(&current) {
            println!("reached loop, quitting");
            return None;
        } else if current >= program.len() {
            println!("reached eof, done");
            return Some(accumulator);
        }
    }
}

fn get_operation<'a, T>(mut iter: T) -> Option<Operation>
where
    T: Iterator<Item = &'a str>,
{
    match iter.next().unwrap() {
        "acc" => Some(Operation::Acc),
        "jmp" => Some(Operation::Jmp),
        "nop" => Some(Operation::Nop),
        _ => None,
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

fn get_argument<'a, T>(mut iter: T) -> Option<i32>
where
    T: Iterator<Item = &'a str>,
{
    match iter.next().unwrap().parse::<i32>() {
        Ok(arg) => Some(arg),
        Err(_) => None,
    }
}
