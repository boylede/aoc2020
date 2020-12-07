use crate::PartResult;
use std::collections::HashMap;

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
            println!("{}: {:?}\n", bag_type, children);

            (bag_type, children)
        })
        .collect();
    Ok("".to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    Ok(" ".to_string())
}
