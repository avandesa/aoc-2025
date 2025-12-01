use std::ops::Add;

use crate::solution::Solution;

#[derive(Clone, Copy, Debug)]
struct Instr(i32);

impl Instr {
    fn parse(line: &str) -> Self {
        let (dir, dist) = line.split_at(1);

        let dir = match dir {
            "L" => -1,
            "R" => 1,
            other => panic!("invalid direction: {other}"),
        };

        let dist = dist.parse::<i32>().expect("invalid distance");

        Self(dist * dir)
    }
}

#[derive(Clone, Copy, Debug)]
struct Safe {
    pos: i32,
    crossed_zero_times: usize,
}

impl Safe {
    fn iter<I: Iterator<Item = Instr>>(self, instrs: I) -> SafeIter<I> {
        SafeIter { safe: self, instrs }
    }

    fn is_zero(&self) -> bool {
        self.pos == 0
    }

    fn crossed_zero_times(self) -> usize {
        self.crossed_zero_times
    }
}

impl Default for Safe {
    fn default() -> Self {
        Self {
            pos: 50,
            crossed_zero_times: 0,
        }
    }
}

impl Add<Instr> for Safe {
    type Output = Self;

    fn add(self, Instr(dist): Instr) -> Self::Output {
        let mut pos = self.pos + dist;

        let crossed_zero_times = if self.pos == 0 && (-99..0).contains(&pos) {
            0
        } else if pos.is_negative() {
            let czt = (pos / 100).unsigned_abs() as usize;
            if pos % 100 == 0 { czt } else { czt + 1 }
        } else if pos.is_positive() {
            (pos / 100) as usize
        } else {
            1
        };

        pos %= 100;
        if pos.is_negative() {
            pos += 100;
        }

        Self {
            pos,
            crossed_zero_times,
        }
    }
}

struct SafeIter<I: Iterator<Item = Instr>> {
    safe: Safe,
    instrs: I,
}

impl<I: Iterator<Item = Instr>> Iterator for SafeIter<I> {
    type Item = Safe;

    fn next(&mut self) -> Option<Self::Item> {
        let instr = self.instrs.next()?;
        self.safe = self.safe + instr;
        Some(self.safe)
    }
}

#[derive(Debug)]
pub struct Day01 {
    instructions: Vec<Instr>,
}

impl Solution for Day01 {
    fn with_input(input: String) -> Self {
        let instructions = input.lines().map(Instr::parse).collect();
        Self { instructions }
    }

    fn part1(&self) -> String {
        Safe::default()
            .iter(self.instructions.iter().copied())
            .filter(Safe::is_zero)
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        Safe::default()
            .iter(self.instructions.iter().copied())
            .map(Safe::crossed_zero_times)
            .sum::<usize>()
            .to_string()
    }
}
