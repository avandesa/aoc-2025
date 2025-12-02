use crate::solution::Solution;

#[derive(Clone, Copy, Debug)]
struct IdRange(u64, u64);

impl IdRange {
    fn parse(range: &str) -> Self {
        let (start, end) = range.split_once('-').unwrap();
        Self(start.parse().unwrap(), end.parse().unwrap())
    }

    fn ids(&self) -> impl Iterator<Item = Id> {
        (self.0..=self.1).map(Id)
    }
}

#[derive(Clone, Copy, Debug)]
struct Id(u64);

impl Id {
    fn contains_repeat(&self) -> bool {
        let s = self.0.to_string();

        if !s.len().is_multiple_of(2) {
            return false;
        }

        let (left, right) = s.split_at(s.len() / 2);

        left == right
    }
}

#[derive(Debug)]
pub struct Day02 {
    ranges: Vec<IdRange>,
}

impl Solution for Day02 {
    fn with_input(input: String) -> Self {
        let ranges = input.trim().split(',').map(IdRange::parse).collect();
        Self { ranges }
    }

    fn part1(&self) -> String {
        self.ranges
            .iter()
            .flat_map(IdRange::ids)
            .filter_map(|id| id.contains_repeat().then_some(id.0))
            .sum::<u64>()
            .to_string()
    }
}
