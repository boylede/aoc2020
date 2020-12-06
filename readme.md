# Advent of Code 2020

## Day 1
6800 ns / 762100 ns

A good starting puzzle, basically a counting excersize to warm up for whatever this year's AOC brings.

## Day 2
135900 ns / 134300 ns

A relatively simple day, I'm getting the sense that we are getting prepped to do a lot of custom deserialization this year.

## Day 3
252000 ns / 271000 ns

I initially wrote this day with a much more complicated solution that used a Coordinate type with a custom Hash implementation, instead of just using modulo on the x value. Halfway through I realized it would be simpler just to write the math where its actually used instead of trying to abstract over it. 

## Day 4
878900 ns / 934500 ns

This day was interesting for the strange non-deterministic serialization that places map keys across any number of lines, requiring a map to be built pair by pair, and finalized when an empty line is encountered. 

I got to use the PhantomData pattern I had initially planned to use for Day 3, which is fun though overkill for this use.
~~~Rust
let passports: Passports<Careless> = lines.iter().collect();
~~~

## Day 5
14900 ns / 26100 ns

The salient bit of code for part 2 is
~~~Rust
let mut seats: Vec<u32> = lines
        .iter()
        .map(|seat| {
            seat.chars()
                .enumerate()
                .filter(|(_, c)| *c == 'B' || *c == 'R')
                .fold(0, |a, (i, _)| a + (1 << (9 - i)))
        })
        .collect();
    seats.sort();
    let my_seat: u32 = seats
        .iter()
        .zip(seats.iter().skip(1))
        .filter_map(|(&a, &b)| if b > a + 1 { Some(a + 1) } else { None })
        .next()
        .unwrap();
~~~
I had a more succinct solution for parsing the seat number but it was wasteful in both cycles and allocations so I replaced it with a version that just builds the integer by bit-manipulation.

Finding the empty seat is easier than I initially thought, but it requires the array to be sorted and then you can just look for gaps by comparing sequential values.
