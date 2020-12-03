use crate::PartResult;
use std::collections::HashMap;

pub fn part1(lines: &Vec<String>) -> PartResult {
    let map = parse_input(&lines);
    let slope = Coord(3, 1);
    let points = count_trees(&map, slope);
    Ok(points.to_string())
}

pub fn part2(lines: &Vec<String>) -> PartResult {
    let map = parse_input(&lines);
    let trees = count_trees(&map, Coord(1, 1))
        * count_trees(&map, Coord(3, 1))
        * count_trees(&map, Coord(5, 1))
        * count_trees(&map, Coord(7, 1))
        * count_trees(&map, Coord(1, 2));

    Ok(trees.to_string())
}

fn count_trees(map: &HashMap<Coord, Tile>, slope: Coord) -> usize {
    slope
        .into_iter()
        // .inspect(|c| println!("{:?}", c))
        .map(|c| map[&c])
        .filter(|&t| t == Tile::Tree)
        .count()
}

fn parse_input(lines: &Vec<String>) -> HashMap<Coord, Tile> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            let tiles: HashMap<Coord, Tile> = line
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    let tile = match c {
                        '#' => Tile::Tree,
                        '.' => Tile::Clear,
                        _ => panic!("invalid input"),
                    };
                    (Coord(x, y), tile)
                })
                .collect();
            tiles
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

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Tile {
    Clear,
    Tree,
}
