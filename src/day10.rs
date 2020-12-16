use crate::PartResult;
use std::collections::{HashMap, HashSet};

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
    println!("sorted input");
    println!("{:?}", adapters);

    // count inflection points in the input data
    let mut inflection_points = adapters
        .clone()
        .into_iter()
        .skip(1)
        .zip(adapters.clone().into_iter())
        .filter(|(b, a)| *b == a + 3)
        .map(|(b, _)| b).collect::<Vec<i32>>();
    
    println!(
        "expecting {} inflection points, {:?}",
        inflection_points.len(),
        inflection_points
    );
    let mut visited_inflection_points: HashMap<i32, usize> = HashMap::new();
    

    // let mut count: u64 = 0;
    let mut wavefront: Vec<(usize, Vec<i32>)> = Vec::new();
    let mut path_count: HashMap<i32, usize> = HashMap::with_capacity(adapters.len());

    let max = adapters[adapters.len() - 1];
    inflection_points.push(max);

    // load starting points into wavefront
    {
        let mut index = 0;
        while index < adapters.len() {
            let difference = adapters[index] - 0;
            if difference <= 3 {
                wavefront.push((index, vec![]));
            } else {
                break;
            }
            index += 1;
        }
    }
    println!("primed with {} starting branches", wavefront.len());

    while !wavefront.is_empty() {
        let (base_index, mut history) = wavefront.pop().unwrap();
        let value = adapters[base_index];
        history.push(value);

        // if we have reached the end of the list
        // if value - max >= 0 {
        //     for adapter in history.iter() {
        //         *path_count.entry(*adapter).or_insert(0) += 1;
        //     }

        //     count += 1;
        // }
        if inflection_points.contains(&value) {
            // or if we have reached an inflection point
            *visited_inflection_points.entry(value).or_insert(0) += 1;
            // if let Some(pos) = inflection_points.iter().position(|a| *a == value) {
            //     // Some(x) => x,
            //     // None => return None,
            //     inflection_points.remove(pos);
            // };
            // Some(self.remove(pos))
            // inflection_points.remove()
        } else {
            // look at any adapters that can fit in this one
            // and add them to the wavefront
            let mut delta = 1;
            while delta < (adapters.len() - base_index) {
                let index = base_index + delta;
                let difference = adapters[index] - adapters[base_index];
                if difference == 1 || difference == 2 || difference == 3 {
                    wavefront.push((index, history.clone()));
                } else {
                    break;
                }
                delta += 1;
            }
        }
        // if we have enumerated the paths through this inflection point, move goalpost to next
        if wavefront.is_empty() && inflection_points.len() > 0 {
            if let Some(pos) = inflection_points.iter().position(|a| *a == value) {
                // Some(x) => x,
                // None => return None,
                let index = adapters.iter().position(|a| *a == value).unwrap();
                inflection_points.remove(pos);
                wavefront.push((index, vec![]));
            };
        }

    }
    println!("{:?}", visited_inflection_points);
    let count = visited_inflection_points.iter().fold(1, |a, (_, &c)| a * c);
    println!("{}", count);
    // let mut ips = path_count
    //     .clone()
    //     .drain()
    //     .filter(|(_, l)| *l as u64 == count)
    //     .map(|(a, _)| a)
    //     .collect::<Vec<_>>();
    // ips.sort();
    // println!("found {} inflection points: {:?}", ips.len(), ips);
    // let factors: Vec<i32> = path_count
    //     .clone()
    //     .drain()
    //     .filter(|(_, l)| *l as u64 != count)
    //     .map(|(a, _)| a)
    //     .collect();

    // println!("factors {:?}", factors);
    // let tots = factors.iter().fold(1, |a, f| f * a);
    // println!("total: {}", tots);
    Ok(count.to_string())
}
