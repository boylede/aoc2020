use crate::PartResult;
use std::collections::HashMap;

pub fn part1(lines: &Vec<String>) -> PartResult {
    let mut direction = 0; // heading in degrees. 0 will be east
    let destination = lines
        .iter()
        .map(|line| {
            // parse the input into a tuple of command and value
            let mut chars = line.chars();
            let command = chars.next().unwrap();
            let value: String = chars.collect();
            (command, value.parse::<i32>().unwrap())
        })
        // .inspect(|(c, v)|println!("command {} value {}", c, v))
        .map(|(c, v)| {
            // add in the current direction to every element
            if c == 'R' {
                direction -= v;
            } else if c == 'L' {
                direction += v;
            }
            direction += 360;
            direction %= 360;
            (c, v, direction)
        })
        .filter_map(|(c, v, d)| {
            // map the c,v,d values to a heading and value
            match c {
                'N' => Some((90, v)),
                'S' => Some((270, v)),
                'E' => Some((0, v)),
                'W' => Some((180, v)),
                'F' => Some((d, v)),
                _ => None,
            }
        })
        // .inspect(|(h, d)|println!("heading {} distance {}", h, d))
        .fold((0, 0), |p, (h, d)| {
            // sum the changes in position given the heading, we can assume only 4 directions used
            match h {
                0 => (p.0 + d, p.1),
                90 => (p.0, p.1 + d),
                180 => (p.0 - d, p.1),
                270 => (p.0, p.1 - d),
                _ => panic!("input doesn't have angular headings :)"),
            }
        });
    // .count();
    let result = destination.0.abs() + destination.1.abs();
    Ok(result.to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    let mut waypoint = (10, 1);
    let destination = lines
        .iter()
        .map(|line| {
            // parse the input into a tuple of command and value
            let mut chars = line.chars();
            let command = chars.next().unwrap();
            let value: String = chars.collect();
            (command, value.parse::<i32>().unwrap())
        })
        // .inspect(|(c, v)|println!("command {} value {}", c, v))
        .map(|(c, v)| {
            // add in the current waypoint to every element
            match c {
                'N' => waypoint.1 += v,
                'S' => waypoint.1 -= v,
                'E' => waypoint.0 += v,
                'W' => waypoint.0 -= v,
                'R' => match v {
                    0 => (),
                    90 => waypoint = (waypoint.1, -waypoint.0),
                    180 => waypoint = (-waypoint.0, -waypoint.1),
                    270 => waypoint = (-waypoint.1, waypoint.0),
                    _ => (),
                },
                'L' => match v {
                    0 => (),
                    90 => waypoint = (-waypoint.1, waypoint.0),
                    180 => waypoint = (-waypoint.0, -waypoint.1),
                    270 => waypoint = (waypoint.1, -waypoint.0),
                    _ => (),
                },
                'F' => (),
                _ => panic!("invalid command in input"),
            }
            (c, v, waypoint)
        })
        // .inspect(|(c, v, (x, y))|println!("waypoint {}, {}", x, y))
        .filter_map(|(c, v, w)| {
            // map the c,v,w values to a delta position
            match c {
                'F' => Some((w.0 * v, w.1 * v)),
                _ => None,
            }
        })
        // .inspect(|(x, y)|println!("delta x {} delta y {}", x, y))
        .fold((0, 0), |(x, y), (dx, dy)| {
            // sum the delta positions to get a total delta
            (x + dx, y + dy)
        });
    // .count();
    let result = destination.0.abs() + destination.1.abs();
    Ok(result.to_string())
}
