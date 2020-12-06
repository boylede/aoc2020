use crate::PartResult;
use std::collections::HashSet;

pub fn part1(lines: &Vec<String>) -> PartResult {
    let mut plane = vec![];
    {
        let mut group: HashSet<char> = HashSet::new();
        for line in lines.iter() {
            if line.is_empty() {
                plane.push(group.clone());
                group.clear();
            } else {
                let pairs = line.chars();
                for pair in pairs {
                    group.insert(pair);
                }
            }
        }
        if group.len() > 0 {
            plane.push(group.clone());
            group.clear();
        }
    }
    let result: usize = plane.iter().map(|group| group.len()).sum();
    Ok(result.to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    let mut plane = vec![];
    {
        let mut group: Vec<HashSet<char>> = vec![];
        for line in lines.iter() {
            if line.is_empty() {
                plane.push(group.clone());
                group.clear();
            } else {
                let pairs: HashSet<char> = line.chars().collect();
                group.push(pairs);
            }
        }
        if group.len() > 0 {
            plane.push(group.clone());
            group.clear();
        }
    }

    let result: usize = plane
        .iter()
        .map(|group| -> HashSet<char> {
            let mut iter = group.iter();
            let first = iter.next().unwrap().clone();
            iter.fold(first, |acc, person| {
                acc.iter()
                    .filter(|c| person.contains(c))
                    .map(|c| c.to_owned())
                    .collect()
            })
        })
        .map(|group| group.len())
        .sum();
    Ok(result.to_string())
}
