use crate::PartResult;

pub fn part1(lines: &Vec<String>) -> PartResult {
    let best_seat = lines
        .iter()
        .map(|seat| {
            seat.chars()
                .rev()
                .enumerate()
                .filter(|(_, c)| *c == 'B' || *c == 'R')
                .fold(0, |a, (i, _)| a | (1 << i))
        })
        .max()
        .unwrap();
    Ok(best_seat.to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    let mut seats: Vec<u32> = lines
        .iter()
        .map(|seat| {
            // let count = seat.len() - 1;
            seat.chars()
                .rev()
                .enumerate()
                .filter(|(_, c)| *c == 'B' || *c == 'R')
                .fold(0, |a, (i, _)| a | (1 << i))
        })
        .collect();
    seats.sort_unstable();
    let my_seat: u32 = seats
        .iter()
        .zip(seats.iter().skip(1))
        .filter_map(|(&a, &b)| if b > a + 1 { Some(a + 1) } else { None })
        .next()
        .unwrap();
    Ok(my_seat.to_string())
}
