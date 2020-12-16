use crate::PartResult;
use std::collections::HashMap;

// we make some pretty extensive assumptions here.
pub fn part1(lines: &Vec<String>) -> PartResult {
    let mut adapters = lines
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    adapters.sort();
    let groups = adapters
        .iter()
        .zip(adapters.iter().skip(1))
        .map(|(c, n)| n - c);
    // counting is off by one so instead of fixing just add one.
    let ones = groups.clone().filter(|n| *n == 1).count() + 1;
    let threes = groups.clone().filter(|n| *n == 3).count() + 1;
    let result = ones * threes;
    // println!("found {} ones and {} threes", ones, threes);
    Ok(result.to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    let mut adapters = lines
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    adapters.sort();
    // find inflection points in the input data
    // where we know all paths must cross
    let mut inflection_points = adapters
        .iter()
        .skip(1)
        .zip(adapters.iter())
        .filter(|(b, a)| **b == **a + 3)
        .map(|(b, _)| *b)
        .collect::<Vec<i32>>();
    let mut visited_inflection_points: HashMap<i32, usize> = HashMap::new();
    let mut wavefront: Vec<usize> = Vec::new();
    let max = adapters[adapters.len() - 1];
    inflection_points.push(max);
    // load starting points into wavefront
    {
        let mut index = 0;
        while index < adapters.len() {
            let difference = adapters[index] - 0;
            if difference <= 3 {
                wavefront.push(index);
            } else {
                break;
            }
            index += 1;
        }
    }
    while !wavefront.is_empty() {
        let base_index = wavefront.pop().unwrap();
        let value = adapters[base_index];

        if inflection_points.contains(&value) {
            // if we have reached an inflection point
            // we'll stop looking and increment the number of paths
            // which get to this inflection point
            *visited_inflection_points.entry(value).or_insert(0) += 1;
        } else {
            // look at any adapters that can fit in this one
            // and add them to the wavefront
            let mut delta = 1;
            while delta < (adapters.len() - base_index) {
                let index = base_index + delta;
                let difference = adapters[index] - adapters[base_index];
                if difference == 1 || difference == 2 || difference == 3 {
                    wavefront.push(index);
                } else {
                    break;
                }
                delta += 1;
            }
        }
        // if we have enumerated all the paths up until this inflection point, move goalpost to next
        if wavefront.is_empty() && inflection_points.len() > 0 {
            let pos = inflection_points.iter().position(|a| *a == value).unwrap();
            let index = adapters.iter().position(|a| *a == value).unwrap();
            inflection_points.remove(pos);
            wavefront.push(index);
        }
    }
    // multiply the number of paths between each inflection point to get the total number of paths
    let count = visited_inflection_points.iter().fold(1, |a, (_, &c)| a * c);
    Ok(count.to_string())
}
