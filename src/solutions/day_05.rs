use std::collections::BTreeMap;

use crate::solution::Solution;

#[derive(Clone, Copy, Debug)]
struct Range(u64, u64);

impl Range {
    fn parse(line: &str) -> Self {
        let (from, to) = line.split_once('-').unwrap();
        Self(from.parse().unwrap(), to.parse().unwrap())
    }
}

#[derive(Debug)]
pub struct Day05 {
    ranges: Vec<Range>,
    ingredients: Vec<u64>,
}

impl Solution for Day05 {
    fn with_input(input: String) -> Self {
        let (ranges, ingredients) = input.split_once("\n\n").unwrap();

        let ranges = ranges.lines().map(Range::parse).collect();
        let ingredients = ingredients.lines().map(|l| l.parse().unwrap()).collect();

        Self {
            ranges,
            ingredients,
        }
    }

    fn part1(&self) -> String {
        let mut ingredients_map = self
            .ingredients
            .iter()
            .copied()
            .map(|i| (i, false))
            .collect::<BTreeMap<_, _>>();

        for Range(from, to) in &self.ranges {
            for (_, is_fresh) in ingredients_map.range_mut(from..=to) {
                *is_fresh = true;
            }
        }

        ingredients_map.values().filter(|v| **v).count().to_string()
    }
}
