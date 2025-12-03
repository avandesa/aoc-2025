use std::io::Write;

use itertools::Itertools;

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
    fn contains_double_repeat(&self, buf: &mut Vec<u8>) -> bool {
        buf.clear();
        write!(buf, "{}", self.0).unwrap();

        if !buf.len().is_multiple_of(2) {
            return false;
        }

        let (left, right) = buf.split_at(buf.len() / 2);

        left == right
    }

    fn contains_any_repeat(&self, buf: &mut Vec<u8>) -> bool {
        if self.contains_double_repeat(buf) {
            return true;
        }

        // buffer has already been cleared and written

        for num_parts in 2..=buf.len() {
            if !buf.len().is_multiple_of(num_parts) {
                continue;
            }

            if buf.chunks(buf.len() / num_parts).all_equal() {
                return true;
            }
        }

        false
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
        let mut buf = Vec::with_capacity(10);

        self.ranges
            .iter()
            .flat_map(IdRange::ids)
            .filter_map(|id| id.contains_double_repeat(&mut buf).then_some(id.0))
            .sum::<u64>()
            .to_string()
    }

    fn part2(&self) -> String {
        let mut buf = Vec::with_capacity(10);
        self.ranges
            .iter()
            .flat_map(IdRange::ids)
            .filter_map(|id| id.contains_any_repeat(&mut buf).then_some(id.0))
            .sum::<u64>()
            .to_string()
    }
}
