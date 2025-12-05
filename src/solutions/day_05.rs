use rangemap::RangeInclusiveSet;

use crate::solution::Solution;

#[derive(Debug)]
pub struct Day05 {
    ranges: RangeInclusiveSet<u64>,
    ingredients: Vec<u64>,
}

impl Solution for Day05 {
    fn with_input(input: String) -> Self {
        let (ranges, ingredients) = input.split_once("\n\n").unwrap();

        let ranges = ranges
            .lines()
            .map(|line| {
                let (from, to) = line.split_once('-').unwrap();
                from.parse().unwrap()..=to.parse().unwrap()
            })
            .collect();
        let ingredients = ingredients.lines().map(|l| l.parse().unwrap()).collect();

        Self {
            ranges,
            ingredients,
        }
    }

    fn part1(&self) -> String {
        self.ingredients
            .iter()
            .filter(|i| self.ranges.contains(i))
            .count()
            .to_string()
    }
}
