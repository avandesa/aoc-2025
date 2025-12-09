use itertools::Itertools;

use crate::solution::Solution;

#[derive(Clone, Copy, Debug)]
struct Tile(u64, u64);

impl Tile {
    fn parse(line: &str) -> Self {
        let (x, y) = line.split_once(',').unwrap();
        Self(x.parse().unwrap(), y.parse().unwrap())
    }

    fn area(&self, other: &Self) -> u64 {
        (self.0.abs_diff(other.0) + 1) * (self.1.abs_diff(other.1) + 1)
    }
}

#[derive(Debug)]
pub struct Day09 {
    red_tiles: Vec<Tile>,
}

impl Solution for Day09 {
    fn with_input(input: String) -> Self {
        let red_tiles = input.lines().map(Tile::parse).collect();
        Self { red_tiles }
    }

    fn part1(&self) -> String {
        self.red_tiles
            .iter()
            .tuple_combinations()
            .map(|(a, b)| a.area(b))
            .max()
            .unwrap()
            .to_string()
    }
}
