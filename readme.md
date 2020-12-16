# Advent of Code 2020

## Day 1
6800 ns / 762100 ns

A good starting puzzle, basically a counting excersize to warm up for whatever this year's AOC brings.

~~~Rust
// the first lines create an iterator that loops over all
// sets of three items from the input.
expenses
    .iter()
    .flat_map(|&a| std::iter::repeat(a).zip(expenses.iter()))
    .flat_map(|(a, &b)| {
        std::iter::repeat(a)
            .zip(std::iter::repeat(b))
            .zip(expenses.iter())
    })
// then we destructure that nested tuple and
// add the sum alongside it
    .map(|((a, b), &c)| (a + b + c, a, b, c))
// eliminating numbers that don't match our test predicate
    .filter(|(n, _, _, _)| *n == 2020)
// and finally getting the product of the set that matches
    .map(|(_, a, b, c)| a * b * c)
    .next()
    .unwrap();
// note that this solution does not attempt to avoid having
// 3 of the same number to do that we would enumerate the
// input iterator and filter out matching indexes but my
// input did not require this step.
~~~

## Day 2
135900 ns / 134300 ns

A relatively simple day, I'm getting the sense that we are getting prepped to do a lot of custom deserialization this year.

~~~Rust
let number = password
    .chars()
    .filter(|&pc| pc == letter)
    .count() as i32;
number >= a && number <= b
~~~

## Day 3
252000 ns / 271000 ns

I initially wrote this day with a much more complicated solution that used a Coordinate type with a custom Hash implementation, instead of just using modulo on the x value. Halfway through I realized it would be simpler just to write the math where its actually used instead of trying to abstract over it. 

~~~Rust
// The central part of this day is counting items that
// appear in both an Iterator and the HashSet.
slope.into_iter().filter(|c| map.contains(c)).count()

// To keep things simple I made a separate Iterator type
// that produces coordinates given the input slope until
// it reaches the bottom of the map.
impl Iterator for CoordIter {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        let x = (self.current.0 + self.delta.0) % 31;
        let y = self.current.1 + self.delta.1;
        if y > 322 {
            None
        } else {
            let c = Coord(x, y);
            self.current = c;
            Some(c)
        }
    }
}
~~~

## Day 4
878900 ns / 934500 ns

This day was interesting for the strange non-deterministic serialization that places map keys across any number of lines, requiring a map to be built pair by pair, and finalized when an empty line is encountered. 

I got to use the PhantomData pattern I had initially planned to use for Day 3, which is fun though overkill for this use.
~~~Rust
// the central part of the code uses built-in traits so
// the actual call site is very plain
let passports: Passports<Careless> = lines.iter().collect();

// to allow the different passport validation modes
// to both be created from the same IntoIterator,
// I include a validation mode generic type so the
// call site above can "request" any validation type
// just by changing the type it receives
struct Passports<M> {
    inner: Vec<Passport>,
    _mode: PhantomData<M>,
}
// The actual modes don't contain any data
struct Careless;
// they are just placeholders to reference a function
impl Mode for Careless {
    fn create_passport(map: HashMap<String, String>) -> Passport {
        let valid = REQUIRED_KEYS
            .iter()
            .all(|key| map.contains_key(key.to_owned()));
        use Passport::*;
        match valid {
            true => Valid(map),
            false => Invalid(map),
        }
    }
}
// the function can then be accessed from any other
// place that uses that type
impl<'a, M: Mode> FromIterator<&'a String> for Passports<M> {
    fn from_iter<I: IntoIterator<Item = &'a String>>(iter: I) -> Self {
// .. snip .. //
                let passport = M::create_passport(current.clone());
// .. snip .. //
        Passports {
            inner,
            _mode: <PhantomData<M> as Default>::default(),
        }
    }
}
// so part 2 looks just like part 1 but with a different type
// and a different backing function
let passports: Passports<Strict> = lines.iter().collect();
~~~

## Day 5
14900 ns / 26100 ns

The salient bit of code for part 2 is
~~~Rust
let mut seats: Vec<u32> = lines
        .iter()
        .map(|seat| {
            // the seat numbers we want are just binary encoded as strings
            seat.chars()
                // by reversing the iterator we can shift our bits into
                // place more easily later
                .rev()
                .enumerate()
                // B and R and used as 1 bits
                .filter(|(_, c)| *c == 'B' || *c == 'R')
                // this math moves the bit into place
                .fold(0, |a, (i, _)| a | (1 << i))
        })
        .collect();
    // to find the missing seat it is easiest if we just sort the array
    seats.sort();
    // and then we can look for sequential values with a delta more than 1
    let my_seat: u32 = seats
        .iter()
        .zip(seats.iter().skip(1))
        .filter_map(|(&a, &b)| if b > a + 1 { Some(a + 1) } else { None })
        .next()
        .unwrap();
~~~
I had a more succinct solution for parsing the seat number but it was wasteful in both cycles and allocations so I replaced it with the version above that builds the integer by bit-manipulation.

~~~Rust
lines
    .iter()
    .map(|seat| seat.replace(ZERO_CHARS, &"0").replace(ONE_CHARS, &"1").parse::<u32>().unwrap())
    .collect();
~~~

Finding the empty seat is easier than I initially thought, but it requires the array to be sorted. Then you can just look for gaps by comparing each value to the next one.

## Day 6
367700 ns / 1 ms

I tried to actually do this one quickly. I got Part 1 in 7:26, not enough to get on the global leaderboard but certainly my quickest solve of any AOC. Then it took me 20 minutes to figure out what i was doing wrong in part 2. The parse code is very simular to day 4, more or less a copy paste job but I ommitted the unneeded complexity I had put in day 4. Part 1 just sums each HashSet's length. Part 2 requires a little different parsing since it wants to know what was on each line individually not just what is in each group of lines. Unfortunately I am looping over the groups a lot so part 2 takes a whole 1ms.


## Day 7
523200 ns / 378800 ns

This day was a bear for me. It isn't real complicated, but it took me almost an hour to get the parsing finished for part 1. The math is really simple, you keep a list of unvisited containing bags and for every one look at bags that could contain it, and add them to the list. Each time you add one to the list you also increment a counter. This would be a potential infinite loop but AOC's input does not contain loops.

~~~Rust
let mut outer_bags: HashSet<&str> = HashSet::new();
let mut unvisited_bags: Vec<&str> = vec![];
unvisited_bags.push("shiny gold");
while unvisited_bags.len() > 0 {
    let next_bag = unvisited_bags.pop().unwrap();
    if let Some(bag_type) = reverse_rules.get(&next_bag) {
        for bag in bag_type {
            if !outer_bags.contains(bag) {
                unvisited_bags.push(bag.clone());
                outer_bags.insert(bag.clone());
            }
        }
    }
}
~~~

Part 2 had different parsing requirements, which were simpler. Using the same visitor loop as part 1, you just need to keep track of how many bags are needed and add them instead of just incrementing the counter.

~~~Rust
let mut bags = 0;
let mut unvisited_bags: Vec<(usize, &str)> = vec![];
unvisited_bags.push((1, "shiny gold"));
while unvisited_bags.len() > 0 {
    let (next_count, next_bag) = unvisited_bags.pop().unwrap();
    let children = rules.get(&next_bag).unwrap();
    for (child_num, child_color) in children.iter() {
        let meta = next_count * child_num;
        unvisited_bags.push((meta, child_color));
        bags += meta;
    }
}
~~~

## Day 8
66700 ns / 2 ms

## Day 9
51800 ns / 36 ms

## Day 10
18300 ns / 15200 ns

This day was tough, and a real treat to solve. The instructions hinted that a brute force solution would not be feasible, so I am glad that I was able to find a technique that would reduce the number of loops needed. My solution for part 2 is faster than my solution to part 1. My initial attempt only tried to count the paths by brute force, at it took me some time to make sure it was valid on the test cases. But the actual input results in an astronomically longer calculation so as expected I had to find a new way. looking at some debug prints, I realized that the input data does contain one saving grace: there are certain pivot points through which all paths must walk. Knowing that I realized I could count the number of paths between each pivot point and simply multiply them all together. Writing that code took less time than debugging the brute force version.
