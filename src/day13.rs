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
    let rules = lines[1]
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
        .collect::<Vec<(i64, i64)>>();

    let (max_i, max) = rules.iter().max_by(|(i, b), (ii, bb)| b.cmp(bb)).unwrap();
    println!("rules: {:?}", rules);
    let slope: i64 = rules.iter().map(|(_, b)| b).product();

    println!("determined slope: {}", slope);
    let zeroth = rules
        .iter()
        .filter(|(i, _)| *i == 0)
        .map(|(_, b)| b)
        .next()
        .unwrap();
    println!("zeroth: {}", zeroth);
    // let rise;
    let mut successes = vec![];
    let mut n = 1;
    let mut count = 0;
    loop {
        n += 1;
        
        let test_number = (max * n) + max_i;
        let correct = rules
            .iter()
            .all(|(offset, bus)| test_number % bus == *offset);
        // print!(" {} ", test_number);
        if correct {
            println!("{{{},{}}},", count, test_number);
            successes.push(test_number);
            // rise = test_number;
            // break;
            count +=1;
        }
        if count > 10 {
            println!("");
            break;
        }
    }
    let rise = successes.iter().next().unwrap();
    println!("slope: {}, rise: {}, result: {}", slope, rise, slope - rise);


    Ok((slope - rise).to_string())
}
