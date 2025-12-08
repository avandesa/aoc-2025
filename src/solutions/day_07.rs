use std::fmt::Display;

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

impl Display for ManifoldState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in &self.0 {
            match s {
                State::Empty => '.'.fmt(f)?,
                State::Beam => '|'.fmt(f)?,
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Day07 {
    width: usize,
    beam_source: usize,
    splitter_locations: Vec<Vec<usize>>,
}

impl Solution for Day07 {
    fn with_input(input: String) -> Self {
        let width = input.find('\n').unwrap();
        let beam_source = input.find('S').unwrap();

        let splitter_locations = input
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
            splitter_locations,
        }
    }

    fn part1(&self) -> String {
        let mut state = ManifoldState::new(self.width, self.beam_source);
        let mut activations = 0;
        for splitters in &self.splitter_locations {
            activations += state.split(splitters);
        }

        activations.to_string()
    }
}
