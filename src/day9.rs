use crate::PartResult;

pub fn part1(lines: &Vec<String>) -> PartResult {
    let mut signal = lines.iter().map(|line| line.parse::<u64>().unwrap());
    let mut preamble = vec![];
    // load preamble
    for _ in 0..25 {
        preamble.push(signal.next().unwrap());
    }
    // println!("finished loading preamble: {:?}", preamble);
    let result = loop {
        let next_num = signal.next().unwrap();

        if !validate(&preamble, next_num) {
            // println!("{} doesnt validate", next_num);
            break next_num;
        } else {
            // println!("{} validates", next_num);
            preamble.remove(0);
            preamble.push(next_num);
        }
    };
    Ok(result.to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    let test_stream = lines.iter().map(|line| line.parse::<u64>().unwrap());
    let mut signal = test_stream.clone();
    let mut preamble = vec![];
    for _ in 0..25 {
        preamble.push(signal.next().unwrap());
    }
    // println!("finished loading preamble: {:?}", preamble);
    let test = loop {
        let next_num = signal.next().unwrap();

        if !validate(&preamble, next_num) {
            // println!("{} doesnt validate", next_num);
            break next_num;
        } else {
            // println!("{} validates", next_num);
            preamble.remove(0);
            preamble.push(next_num);
        }
    };

    let test_stream: Vec<u64> = test_stream.collect();

    let mut result = None;
    for start in 0..test_stream.len() {
        for length in 1..test_stream.len() - start {
            if validate_stream(start, length, &test_stream, test) {
                // println!("found key run of numbers at {} running {} long", start, length);
                let min: u64 = **(&test_stream[start..start + length].iter().min().unwrap());
                let max: u64 = **(&test_stream[start..start + length].iter().max().unwrap());
                // println!("{} + {} = {}", min, max, min + max);
                if let None = result {
                    result = Some(min + max);
                } else {
                    println!(
                        "found additional matching range at {}-{}",
                        start,
                        start + length
                    );
                }
            }
        }
    }

    Ok(result.unwrap().to_string())
}

fn validate(preamble: &[u64], num: u64) -> bool {
    for (i, num_a) in preamble.iter().enumerate() {
        for (j, num_b) in preamble.iter().enumerate() {
            if i != j && num_a != num_b {
                if num_a + num_b == num {
                    return true;
                }
            }
        }
    }
    false
}

fn validate_stream(start: usize, length: usize, stream: &Vec<u64>, test: u64) -> bool {
    let numbers = &stream[start..start + length];
    let sum = numbers.iter().sum();
    test == sum
}
