use crate::PartResult;
use std::collections::HashSet;

pub fn part1(lines: &Vec<String>) -> PartResult {
    let trees = parse_input(&lines);
    let points = count_trees(&trees, Coord(3, 1));
    Ok(points.to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    let trees = parse_input(&lines);
    let trees = count_trees(&trees, Coord(1, 1))
        * count_trees(&trees, Coord(3, 1))
        * count_trees(&trees, Coord(5, 1))
        * count_trees(&trees, Coord(7, 1))
        * count_trees(&trees, Coord(1, 2));
    Ok(trees.to_string())
}

fn count_trees(map: &HashSet<Coord>, slope: Coord) -> usize {
    slope.into_iter().filter(|c| map.contains(c)).count()
}

fn parse_input(lines: &Vec<String>) -> HashSet<Coord> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| -> Vec<Coord> {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| if c == '#' { Some(Coord(x, y)) } else { None })
                .collect()
        })
        .collect()
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Coord(usize, usize);

impl Coord {
    fn into_iter(self) -> CoordIter {
        CoordIter {
            current: Coord(0, 0),
            delta: self,
        }
    }
}

struct CoordIter {
    current: Coord,
    delta: Coord,
}

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
