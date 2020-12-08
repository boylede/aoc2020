use crate::PartResult;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn part1(lines: &Vec<String>) -> PartResult {
    let reverse_rules: HashMap<&str, Vec<&str>> = lines
        .iter()
        .flat_map(|line| {
            let mut tokens = line.split("bags contain");
            let bag_type = tokens.next().unwrap().trim();
            tokens
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.trim())
                .filter_map(|span| {
                    let mut spans = span.splitn(2, ' ');
                    spans
                        .next()
                        .unwrap()
                        .trim()
                        .parse::<usize>()
                        .ok()
                        .map(|num| {
                            (
                                num,
                                spans
                                    .next()
                                    .unwrap()
                                    .trim_end_matches('.')
                                    .trim_end_matches('s')
                                    .trim_end_matches(&" bag"),
                            )
                        })
                })
                .map(|(_, little)| (little, bag_type))
                .collect::<Vec<(&str, &str)>>()
        })
        .fold(HashMap::new(), |mut a, (little, big)| {
            a.entry(little).or_insert(vec![]).push(big);
            a
        });

    let mut outer_bags: HashSet<&str> = HashSet::new();
    let mut unvisited_bags: Vec<&str> = vec![];
    unvisited_bags.push("shiny gold");
    while unvisited_bags.len() > 0 {
        let next_bag = unvisited_bags.pop().unwrap();
        if let Some(bag_type) = reverse_rules.get(&next_bag) {
            for bag in bag_type {
                if !outer_bags.contains(bag) {
                    unvisited_bags.push(bag.clone());
                    outer_bags.insert(bag.clone());
                }
            }
        }
    }
    Ok(outer_bags.len().to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    let rules: HashMap<&str, Vec<(usize, &str)>> = lines
        .iter()
        .map(|line| {
            let mut tokens = line.split("bags contain");
            let bag_type = tokens.next().unwrap().trim();
            let children = tokens
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.trim())
                .filter_map(|span| {
                    let mut spans = span.splitn(2, ' ');
                    spans
                        .next()
                        .unwrap()
                        .trim()
                        .parse::<usize>()
                        .ok()
                        .map(|num| {
                            (
                                num,
                                spans
                                    .next()
                                    .unwrap()
                                    .trim_end_matches('.')
                                    .trim_end_matches('s')
                                    .trim_end_matches(&" bag"),
                            )
                        })
                })
                .collect::<Vec<(usize, &str)>>();
            (bag_type, children)
        })
        .collect();

    let mut bags = 0;
    let mut unvisited_bags: Vec<(usize, &str)> = vec![];
    unvisited_bags.push((1, "shiny gold"));
    while unvisited_bags.len() > 0 {
        let (next_count, next_bag) = unvisited_bags.pop().unwrap();
        if next_count > 0 {
            let children = rules.get(&next_bag).unwrap();
            for (child_num, child_color) in children.iter() {
                if *child_num != 0 {
                    let meta = next_count * child_num;
                    unvisited_bags.push((meta, child_color));
                    bags += meta;
                }
            }
        }
    }
    Ok(bags.to_string())
}
