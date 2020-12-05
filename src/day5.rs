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
    let my_seat: u32 = seats
        .iter()
        .zip(seats.iter().skip(1))
        .filter_map(|(&a, &b)| if b > a + 1 { Some(a + 1) } else { None })
        .next()
        .unwrap();
    Ok(my_seat.to_string())
}
