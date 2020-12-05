use crate::{PartError, PartResult};

pub fn part1(lines: &Vec<String>) -> PartResult {
    let best_seat = lines
        .iter()
        .map(|seat| {
            let mut chars = seat.chars();
            let (row, _) = (&mut chars)
                .take(7)
                .fold((0, 127), |(min, max), c| cut_range(min, max, c));
            let (col, _) = (&mut chars)
                .take(3)
                .fold((0, 7), |(min, max), c| cut_range(min, max, c));
            (row, col)
        })
        .map(|(row, col)| row * 8 + col)
        .max()
        .unwrap();

    Ok(best_seat.to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    let mut seats: Vec<u32> = lines
        .iter()
        .map(|seat| {
            let mut chars = seat.chars();
            let (row, _) = (&mut chars)
                .take(7)
                .fold((0, 127), |(min, max), c| cut_range(min, max, c));
            let (col, _) = (&mut chars)
                .take(3)
                .fold((0, 7), |(min, max), c| cut_range(min, max, c));
            (row, col)
        })
        .map(|(row, col)| row * 8 + col)
        .collect();
    seats.sort();
    let my_seat: u32 = seats[..]
        .windows(2)
        .filter_map(|slice| {
            let empty_seat = slice.get(0).unwrap() + 1;
            if *(slice.get(1).unwrap()) > empty_seat {
                Some(empty_seat)
            } else {
                None
            }
        })
        .next()
        .unwrap();
    Ok(my_seat.to_string())
}

fn cut_range(min: u32, max: u32, determinant: char) -> (u32, u32) {
    let mid = ((max - min) / 2) + min;
    match determinant {
        'F' | 'L' => (min, mid),
        'B' | 'R' => (mid + 1, max),
        _ => panic!("invalid determinant: {}", determinant),
    }
}
