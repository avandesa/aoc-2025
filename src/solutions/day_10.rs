use std::fmt::Debug;

use arrayvec::ArrayVec;
use itertools::Itertools;

use crate::solution::Solution;

const MAX_LIGHTS: usize = 10;
const MAX_BUTTONS: usize = 13;

#[derive(Clone, Default, PartialEq)]
struct Lights(ArrayVec<bool, MAX_LIGHTS>);

impl Lights {
    fn new(size: usize) -> Self {
        let mut lights = ArrayVec::new();
        lights.extend(std::iter::repeat_n(false, size));
        Self(lights)
    }

    fn push_button(&mut self, b: &Button) {
        for i in b.lights_activated() {
            self.0[i] ^= true; // invert via xor
        }
    }
}

impl Lights {
    fn parse(s: &str) -> Self {
        let lights = s
            .strip_prefix('[')
            .unwrap()
            .strip_suffix(']')
            .unwrap()
            .chars()
            .map(|b| match b {
                '.' => false,
                '#' => true,
                _ => panic!("invalid light req: `{b}`"),
            })
            .collect();
        Self(lights)
    }
}

impl Debug for Lights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let required_state = self
            .0
            .iter()
            .map(|b| if *b { '#' } else { '.' })
            .collect::<String>();
        f.debug_tuple("Lights").field(&required_state).finish()
    }
}

#[derive(Clone, PartialEq)]
struct Button(ArrayVec<usize, MAX_LIGHTS>);

impl Button {
    fn parse(s: &str) -> Self {
        let button = s
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split(',')
            .map(|l| l.parse::<usize>().unwrap())
            .collect();

        Self(button)
    }

    fn lights_activated(&self) -> impl Iterator<Item = usize> {
        self.0.iter().copied()
    }
}

impl Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_tuple("Button");
        for light in self.lights_activated() {
            d.field(&light);
        }
        d.finish()
    }
}

#[derive(Clone, Debug)]
struct Machine {
    required_state: Lights,
    buttons: ArrayVec<Button, MAX_BUTTONS>,
    #[allow(unused)]
    joltage_reqs: Vec<u16>,
}

impl Machine {
    fn parse(line: &str) -> Self {
        let mut tokens = line.split_whitespace().peekable();

        let required_state = Lights::parse(tokens.next().unwrap());

        let buttons = tokens
            .peeking_take_while(|t| t.starts_with('('))
            .map(Button::parse)
            .collect();

        let joltage_reqs = tokens
            .next()
            .unwrap()
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        Self {
            required_state,
            buttons,
            joltage_reqs,
        }
    }
}

impl Machine {
    fn shortest_sequence(&self) -> usize {
        for button_set in (0..self.buttons.len()).flat_map(|k| self.buttons.iter().combinations(k))
        {
            let mut lights = Lights::new(self.required_state.0.len());
            for button in &button_set {
                lights.push_button(button);
            }
            if lights == self.required_state {
                return button_set.len();
            }
        }

        panic!("no solution found");
    }
}

#[derive(Debug)]
pub struct Day10 {
    machines: Vec<Machine>,
}

impl Solution for Day10 {
    fn with_input(input: String) -> Self {
        let machines = input.lines().map(Machine::parse).collect();
        Self { machines }
    }

    fn part1(&self) -> String {
        self.machines
            .iter()
            .map(Machine::shortest_sequence)
            .sum::<usize>()
            .to_string()
    }
}
