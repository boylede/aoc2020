use crate::PartResult;
use std::collections::HashMap;

pub fn part1(lines: &Vec<String>) -> PartResult {
    let mut waiting_area: HashMap<Coord, Seat> = lines.iter().enumerate().flat_map(|(y, line)| -> Vec<_>{
        line.chars().enumerate().filter_map(|(x, c)| {
            if c == 'L' {
                Some((Coord(x as i32, y as i32), Seat::Empty))
            } else {
                None
            }
        }).collect()
    }).collect();
    while tick(&mut waiting_area) {}
    let result = count_occupied(&waiting_area);
    Ok(result.to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    Ok("".to_string())
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
struct Coord(i32, i32);


#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
enum Seat {
    Empty,
    Full,
}

fn print_area(area: &HashMap<Coord,Seat>) {
    let (min_x, max_x, min_y, max_y) = range(area);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(seat) = area.get(&Coord(x, y)) {
                if *seat == Seat::Full {
                    print!("#");
                } else {
                    print!("L");
                }
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("\nOccupied: {}", count_occupied(&area));
}

fn range(area: &HashMap<Coord,Seat>) -> (i32, i32, i32, i32) {
    let max_x = *area.iter().map(|(Coord(x,y),v)|x).max().unwrap();
    let max_y = *area.iter().map(|(Coord(x,y),v)|y).max().unwrap();
    let min_x = *area.iter().map(|(Coord(x,y),v)|x).min().unwrap();
    let min_y = *area.iter().map(|(Coord(x,y),v)|y).min().unwrap();
    (min_x, max_x, min_y, max_y)
}

fn occupied(area: &HashMap<Coord,Seat>, coord: &Coord) -> bool {
    area.contains_key(coord) && area.get(coord) == Some(&Seat::Full)
}

fn count_occupied(area: &HashMap<Coord,Seat>) -> usize {
    area.iter().filter(|(_, s)| **s == Seat::Full).count()
}

fn tick(area: &mut HashMap<Coord, Seat>) -> bool {
    let o = area.clone();
    let mut changes: Vec<(Coord, Seat)> = vec![];
    let (min_x, max_x, min_y, max_y) = range(area);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let mut neighbors = 0;
            for nx in 0..3 {
                for ny in 0..3 {
                    let neighbor = Coord(x + nx - 1, y + ny - 1);
                    if neighbor != Coord(x, y) {
                        if occupied(area, &neighbor) {
                            neighbors += 1;
                        }
                    }
                }
            }
            if neighbors >= 4 {
                changes.push((Coord(x, y), Seat::Empty));
            } else if neighbors == 0 {
                changes.push((Coord(x, y), Seat::Full));
            }
        }
    }
    for (coord, seat) in changes.iter() {
        area.entry(*coord).and_modify(|e| *e = *seat);
    }
    o != *area
}