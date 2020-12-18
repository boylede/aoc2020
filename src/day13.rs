use crate::PartResult;
use std::collections::HashMap;

pub fn part1(lines: &Vec<String>) -> PartResult {
    let timestamp = lines[0].parse::<i32>().unwrap();
    let (best_bus, time) = lines[1]
        .split(',')
        .filter_map(|n| n.parse::<i32>().ok())
        .map(|bus| {
            let mut time = bus;
            while time < timestamp {
                time += bus;
            }
            (bus, time)
        })
        .map(|(bus, time)| (bus, time - timestamp))
        .min_by(|(_, time_a), (_, time_b)| time_a.cmp(time_b))
        .unwrap();
    Ok((best_bus * time).to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    let mut rules = lines[1]
        .split(',')
        .enumerate()
        .filter_map(|(i, s)| {
            let n = s.parse::<i64>();
            if let Ok(num) = n {
                Some((i as i64, num))
            } else {
                None
            }
        })
        // .inspect(|(i, b)| println!("bus {} at index {}", b, i))
        // .map(|(i, b)| (i as i64, b))
        // .max_by(|(i,b),(ii,bb)| b.cmp(bb))
        // .unwrap();
        .collect::<Vec<(i64, i64)>>();
    // sort the rules largest multiplier -> smallest
    // rules.sort_by(|(_, b),(_, a)|a.cmp(b));
    let (max_i, max) = rules.iter().max_by(|(i,b),(ii,bb)| b.cmp(bb)).unwrap();
    println!("rules: {:?}", rules);
    let slope: i64 = rules.iter().map(|(_,b)|b).product();
    // let max_tries = u64::MAX / slope as u64;
    // println!("number of tries: {}", i64::MAX / slope);
    println!("determined slope: {}", slope);
    let zeroth = rules.iter().filter(|(i, b)|*i==0).map(|(i,b)|b).next().unwrap();

    println!("zeroth: {}", zeroth);
    // for (offset, bus) in rules.iter() {
    let rise;// = 0;
    let mut n = 0;
    loop {
        n += 1;
        let test_number = (max * n) + max_i;
        let correct = rules.iter().all(|(offset, bus) |{
            (test_number - offset) % bus == 0
            // test_number == (multiplier * bus) + offset
        });
        if correct {
            rise = test_number;
            break;
        }
    }
    println!("slope: {}, rise: {}, result: {}", slope, rise, slope - rise);
    panic!("");
    // }
    let mut i: i64 = -slope;
    let mut count = 0;
    let mut found = false;
    while !found {
        i += zeroth;
        count += 1;
        let test_num: i64 = slope + i;
        // print!("{}: trying {} = {}x + {}", count, test_num, slope, i);
        found = rules.iter().all(|(offset, b)| {
            if (test_num + offset) % b == 0 {
                // print!("|{}|", b);
                true
            } else {
                false
            }
            
        });
        // print!("\n");
        if count > slope {
            panic!("");
        }
    }
    println!("result found: y = {}x + {}", slope, i);
    let result = 
    // if i > 0 {
        slope + i;
    // } else {
    //     -i
    // };
    // println!("y = {}", slope - i);

    Ok(result.to_string())
}
