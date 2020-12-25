use crate::PartResult;
use std::collections::HashMap;

pub fn part1(lines: &Vec<String>) -> PartResult {
    let flip_tiles = lines
        .iter()
        .map(|line| -> (i32, i32, i32) {
            let chars = line.chars().collect::<Vec<char>>();
            let mut directions = vec![];
            let mut position = 0;
            while position < chars.len() {
                let (next, pos) = consume_direction(&chars, position);
                directions.push(next);
                position = pos;
            }
            directions
                .iter()
                .map(|dir| dir.to_coords())
                .fold((0, 0, 0), |a, d| (a.0 + d.0, a.1 + d.1, a.2 + d.2))
        })
        .collect::<Vec<(i32,i32,i32)>>();
    let mut visited: HashMap<(i32,i32,i32), u32> = HashMap::new();
    for tile in flip_tiles.into_iter() {
        *visited.entry(tile).or_insert(0) += 1;
    }
    let black = visited.drain().filter(|(tile, turns)| turns % 2 == 1).count();
    Ok(black.to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    Ok("".to_string())
}

#[derive(Debug)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    fn to_coords(&self) -> (i32, i32, i32) {
        use Direction::*;
        match self {
            East => (1, -1, 0),
            SouthEast => (0, -1, 1),
            SouthWest => (-1, 0, 1),
            West => (-1, 1, 0),
            NorthWest => (0, 1, -1),
            NorthEast => (1, 0, -1),
        }
    }
}

struct Coord(i32, i32, i32);

fn consume_direction(stream: &Vec<char>, position: usize) -> (Direction, usize) {
    let c = stream[position];
    match c {
        's' => {
            let cc = stream[position + 1];
            match cc {
                'e' => (Direction::SouthEast, position + 2),
                'w' => (Direction::SouthWest, position + 2),
                _ => panic!("unexpected input"),
            }
        }
        'n' => {
            let cc = stream[position + 1];
            match cc {
                'e' => (Direction::NorthEast, position + 2),
                'w' => (Direction::NorthWest, position + 2),
                _ => panic!("unexpected input"),
            }
        }
        'e' => (Direction::East, position + 1),
        'w' => (Direction::West, position + 1),
        _ => panic!("unexpected input"),
    }
}
