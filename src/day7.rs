use crate::PartResult;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn part1(lines: &Vec<String>) -> PartResult {
    let rules: HashMap<String, Vec<(usize, String)>> = lines
        .iter()
        .map(|line| {
            let mut tokens = line.split("bags contain");
            let bag_type = tokens.next().unwrap().trim().to_string();
            let children: Vec<(usize, String)> = tokens
                .next()
                .unwrap()
                .split(',')
                .map(|span| {
                    let mut spans = span.split(' ');
                    spans.next(); // skip blank
                    let count = spans.next().unwrap().trim().parse::<usize>().ok();
                    if let Some(num) = count {
                        let bag_type: String = spans
                            .take(2)
                            .map(|word| {
                                let mut w = word.to_string();
                                w.push(' ');
                                w
                            })
                            .collect();
                        let bg = bag_type.trim().to_string();
                        (num, bg)
                    } else {
                        (0, "".to_string())
                    }
                })
                .collect();
            // println!("{}: {:?}\n", bag_type, children);

            (bag_type, children)
        })
        .collect();
    let mut reverse: HashMap<String, Vec<String>> = HashMap::new();
    for (big, littles) in rules.iter() {
        for (n, little) in littles.iter() {
            if *n > 0 {
                reverse.entry(little.clone()).or_insert(vec![]).push(big.to_string());
            }
            
        }
    }
    
    let mut outer_bags: HashSet<String> = HashSet::new();
    let mut unvisited_bags: Vec<String> = vec![];
    unvisited_bags.push("shiny gold".to_string());
    while unvisited_bags.len() > 0 {
        let next_bag = unvisited_bags.pop().unwrap();
        println!("looking for {}", next_bag);
        if let Some(bag_type) = reverse.get(&next_bag) {
            for bag in bag_type {
                if !outer_bags.contains(bag) {
                    println!("  found {}", bag);
                    unvisited_bags.push(bag.clone());
                    outer_bags.insert(bag.clone());
                }
            }
        }

    }
    Ok(outer_bags.len().to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    Ok(" ".to_string())
}
