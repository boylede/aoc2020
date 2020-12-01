use crate::{PartError, PartResult};

pub fn part1(lines: &Vec<String>) -> PartResult {
    let expenses: Vec<i32> = lines
        .iter()
        .map(|line| {
            let num = line.parse::<i32>().unwrap();
            num
        })
        .collect();
    let result = expenses
        .iter()
        .flat_map(|&a| std::iter::repeat(a).zip(expenses.iter()))
        .map(|(a, b)| (a + b, a, b))
        .filter(|(n, _, _)| *n == 2020)
        // .inspect(|t| println!("results: {:?}", t))
        .map(|(_, a, b)| a * b)
        .next()
        .unwrap();
    Ok(result.to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    let expenses: Vec<i32> = lines
        .iter()
        .map(|line| {
            let num = line.parse::<i32>().unwrap();
            num
        })
        .collect();
    let result = expenses
        .iter()
        .flat_map(|&a| std::iter::repeat(a).zip(expenses.iter()))
        .flat_map(|(a, &b)| std::iter::repeat(a).zip(std::iter::repeat(b)).zip(expenses.iter()))
        .map(|((a, b,), c)| (a + b + c, a, b, c))
        .filter(|(n, _, _, _)| *n == 2020)
        // .inspect(|t| println!("results: {:?}", t))
        .map(|(_, a, b, c)| a * b * c)
        .next()
        .unwrap();
    Ok(result.to_string())
}
