use crate::PartResult;

const ZERO_CHARS: &[char] = &['F', 'L'];
const ONE_CHARS: &[char] = &['B', 'R'];

pub fn part1(lines: &Vec<String>) -> PartResult {
    let best_seat = lines
        .iter()
        .map(|seat| seat.replace(ZERO_CHARS, &"0").replace(ONE_CHARS, &"1"))
        .filter_map(|seat| u32::from_str_radix(&seat, 2).ok())
        .max()
        .unwrap();

    Ok(best_seat.to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    let mut seats: Vec<u32> = lines
        .iter()
        .map(|seat| seat.replace(ZERO_CHARS, &"0").replace(ONE_CHARS, &"1"))
        .filter_map(|seat| u32::from_str_radix(&seat, 2).ok())
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
