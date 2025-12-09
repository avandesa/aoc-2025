use std::collections::HashMap;

use itertools::Itertools;

use crate::solution::Solution;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum State {
    Empty,
    Beam,
}

#[derive(Clone, Debug)]
struct ManifoldState(Vec<State>);

impl ManifoldState {
    fn new(width: usize, beam_source: usize) -> Self {
        let mut state = vec![State::Empty; width];
        state[beam_source] = State::Beam;
        Self(state)
    }

    fn split(&mut self, splitters: &[usize]) -> u32 {
        let mut activated = 0;
        for i in splitters.iter().copied() {
            if self.0[i] == State::Beam {
                self.0[i - 1] = State::Beam;
                self.0[i] = State::Empty;
                self.0[i + 1] = State::Beam;
                activated += 1;
            }
        }
        activated
    }
}

#[derive(Debug)]
pub struct Day07 {
    width: usize,
    beam_source: usize,
    splitters: Vec<Vec<usize>>,
}

impl Solution for Day07 {
    fn with_input(input: String) -> Self {
        let width = input.find('\n').unwrap();
        let beam_source = input.find('S').unwrap();

        let splitters = input
            .lines()
            .skip(2)
            .enumerate()
            .filter_map(|(i, l)| {
                if i % 2 != 0 {
                    return None;
                }

                let splitters = l.bytes().positions(|b| b == b'^').collect();
                Some(splitters)
            })
            .collect_vec();

        Self {
            width,
            beam_source,
            splitters,
        }
    }

    fn part1(&self) -> String {
        let mut state = ManifoldState::new(self.width, self.beam_source);
        let mut activations = 0;
        for splitters in &self.splitters {
            activations += state.split(splitters);
        }

        activations.to_string()
    }

    fn part2(&self) -> String {
        let mut map = HashMap::<(usize, usize), u64>::new();

        for (i, row) in self.splitters.iter().enumerate().rev() {
            for j in row {
                let left_child_count = (i..self.splitters.len())
                    .find_map(|i| map.get(&(i, j - 1)))
                    .copied()
                    .unwrap_or(1);
                let right_child_count = (i..self.splitters.len())
                    .find_map(|i| map.get(&(i, j + 1)))
                    .copied()
                    .unwrap_or(1);

                map.insert((i, *j), left_child_count + right_child_count);
            }
        }

        map.get(&(0, self.beam_source)).unwrap().to_string()
    }
}
