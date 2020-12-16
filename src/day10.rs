use crate::PartResult;
use std::collections::{HashSet, HashMap};

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

    let inflections = adapters.clone().into_iter().skip(1).zip(adapters.clone().into_iter()).filter(|(b, a)| *b == a + 3).map(|(b,_)|b);
    println!("expecting {} inflection points, {:?}", inflections.clone().count(), inflections.collect::<Vec<_>>());


    let mut count: u64 = 0;
    let mut wavefront: Vec<(usize, Vec<i32>)> = Vec::new();
    let mut path_count: HashMap<i32, usize> = HashMap::with_capacity(adapters.len());
    // for a in adapters.iter() {
    //     path_count.insert(*a, 0);
    // }
    let max = adapters[adapters.len() - 1];
    // println!(
    //     "looking for paths to {}, {}, or {} jolts",
    //     max + 1,
    //     max + 2,
    //     max + 3
    // );

    {
        let mut index = 0;
        while index < adapters.len() {
            // let index = base_index + delta;
            let difference = adapters[index] - 0;
            if difference == 1 || difference == 2 || difference == 3 {
                // print!(" branching at {} ", adapters[index]);
                wavefront.push((index, vec![]));
            } else {
                // print!("!");
                break;
            }
            index += 1;
        }
    }
    println!("primed with {} starting branches", wavefront.len());


    // wavefront.push((0, vec![]));
    while !wavefront.is_empty() {
        let (base_index, mut history) = wavefront.pop().unwrap();
        let value = adapters[base_index];
        history.push(value);
        // print!("{}\tvisiting {} ", wavefront.len(), value);


        if match value - (max + 3) {
            // v if v < -4 => false,
            -3 => true,
            -2 => true,
            -1 => true,
            // 0 => true,
            // 1 => true,
            // 2 => true,
            // 3 => true,
            _ => {
                // print!("^{}^", v);
                false
            }
        } {
            // print!(" finished path -> {:?}", history);
            // print!(".");
            for adapter in history.iter() {
                *path_count.entry(*adapter).or_insert(0) += 1;
                
                // path_count[index] += 1;
            }
            count += 1;
        }

        let mut delta = 1;
        while delta < (adapters.len() - base_index) {
            let index = base_index + delta;
            let difference = adapters[index] - adapters[base_index];
            if difference == 1 || difference == 2 || difference == 3 {
                // print!(" branching at {} ", adapters[index]);
                wavefront.push((index, history.clone()));
            } else {
                // print!("!");
                break;
            }
            delta += 1;
        }
        // print!("\n");
    }
    // println!("");
    println!("{:?}", path_count);
    let mut ips = path_count.clone().drain().filter(|(_, l)| *l as u64 == count).map(|(a, _)|a).collect::<Vec<_>>();
    ips.sort();
    println!("found {} inflection points: {:?}", ips.len(), ips);
    let factors: Vec<i32> = path_count.clone().drain().filter(|(_, l)| *l as u64 != count).map(|(a,_)|a).collect();
    
    println!("factors {:?}", factors);
    // let tots = factors.iter().fold(1, |a, f| f * a);
    // println!("total: {}", tots);
    Ok(count.to_string())
}
