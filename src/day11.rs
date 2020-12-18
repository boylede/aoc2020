use crate::PartResult;
use std::collections::HashMap;

pub fn part1(lines: &Vec<String>) -> PartResult {
    let mut waiting_area: HashMap<Coord, Square> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| -> Vec<_> {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == 'L' {
                        Some((Coord(x as i32, y as i32), Square::Seat(Seat::Empty)))
                    } else {
                        Some((Coord(x as i32, y as i32), Square::Floor))
                    }
                })
                .collect()
        })
        .collect();
    while tick(&mut waiting_area) {}
    let result = count_occupied(&waiting_area);
    Ok(result.to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    let mut waiting_area: HashMap<Coord, Square> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| -> Vec<_> {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == 'L' {
                        Some((Coord(x as i32, y as i32), Square::Seat(Seat::Empty)))
                    } else {
                        Some((Coord(x as i32, y as i32), Square::Floor))
                    }
                })
                .collect()
        })
        .collect();
    while tick_distant(&mut waiting_area) {}
    let result = count_occupied(&waiting_area);
    Ok(result.to_string())
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
struct Coord(i32, i32);

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
enum Seat {
    Empty,
    Full,
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
enum Square {
    Floor,
    Seat(Seat),
}

fn print_area(area: &HashMap<Coord, Square>) {
    let (min_x, max_x, min_y, max_y) = range(area);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(square) = area.get(&Coord(x, y)) {
                if let Square::Seat(seat) = square {
                    if *seat == Seat::Full {
                        print!("#");
                    } else {
                        print!("L");
                    }
                } else {
                    print!(".");
                }
            } else {
                panic!("map not rectangular?");
            }
        }
        println!("");
    }
    println!("\nOccupied: {}", count_occupied(&area));
}

fn range<T>(area: &HashMap<Coord, T>) -> (i32, i32, i32, i32) {
    let max_x = *area.iter().map(|(Coord(x, y), _)| x).max().unwrap();
    let max_y = *area.iter().map(|(Coord(x, y), _)| y).max().unwrap();
    let min_x = *area.iter().map(|(Coord(x, y), _)| x).min().unwrap();
    let min_y = *area.iter().map(|(Coord(x, y), _)| y).min().unwrap();
    (min_x, max_x, min_y, max_y)
}

fn occupied(area: &HashMap<Coord, Square>, coord: &Coord) -> bool {
    area.contains_key(coord) && area.get(coord) == Some(&Square::Seat(Seat::Full))
}

fn count_occupied(area: &HashMap<Coord, Square>) -> usize {
    area.iter()
        .filter(|(_, s)| **s == Square::Seat(Seat::Full))
        .count()
}

fn tick(area: &mut HashMap<Coord, Square>) -> bool {
    let o = area.clone();
    let mut changes: Vec<(Coord, Square)> = vec![];
    for (coord, square) in area.iter() {
        if let Square::Seat(seat) = square {
            let directions = [-1, 0, 1];
            let neighbors = directions
                .iter()
                .flat_map(|dx: &i32| -> Vec<(i32, i32)> {
                    directions.iter().map(|dy| (*dx, *dy)).collect()
                })
                .filter(|(x, y)| !(*x == 0 && *y == 0))
                .map(|(x, y)| Coord(coord.0 + x, coord.1 + y))
                .filter(|coord| occupied(area, coord))
                .count();
            if neighbors >= 4 {
                changes.push((*coord, Square::Seat(Seat::Empty)));
            } else if neighbors == 0 {
                changes.push((*coord, Square::Seat(Seat::Full)));
            }
        }
    }
    for (coord, seat) in changes.iter() {
        area.entry(*coord).and_modify(|e| *e = *seat);
    }
    o != *area
}

fn tick_distant(area: &mut HashMap<Coord, Square>) -> bool {
    let o = area.clone();
    let mut changes: Vec<(Coord, Square)> = vec![];
    for (coord, square) in area.iter() {
        if let Square::Seat(_) = square {
            let directions = [-1, 0, 1];
            let neighbors = directions
                .iter()
                .flat_map(|dx: &i32| -> Vec<(i32, i32)> {
                    directions.iter().map(|dy| (*dx, *dy)).collect()
                })
                .filter(|(x, y)| !(*x == 0 && *y == 0))
                .map(|(dx, dy)| {
                    let (mut x, mut y) = (coord.0, coord.1);
                    x += dx;
                    y += dy;
                    while area.get(&Coord(x, y)) == Some(&Square::Floor) {
                        x += dx;
                        y += dy;
                    }
                    Coord(x, y)
                })
                .filter(|coord| occupied(area, coord))
                .count();
            if neighbors >= 5 {
                changes.push((*coord, Square::Seat(Seat::Empty)));
            } else if neighbors == 0 {
                changes.push((*coord, Square::Seat(Seat::Full)));
            }
        }
    }
    for (coord, seat) in changes.iter() {
        area.entry(*coord).and_modify(|e| *e = *seat);
    }
    o != *area
}
