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
    light_button_map: LightToButtonMap,
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
            .collect::<ArrayVec<_, _>>();

        let light_button_map = LightToButtonMap::from_buttons(required_state.0.len(), &buttons);

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
            light_button_map,
            joltage_reqs,
        }
    }
}

impl Machine {
    fn smallest_button_set(&self) -> usize {
        let state = AlgState::new(self.required_state.0.len(), &self.buttons);
        self.smallest_button_set_from_light(0, state).unwrap()
    }

    fn smallest_button_set_from_light(&self, light: usize, state: AlgState) -> Option<usize> {
        let matches_required_state = state.lights.0[light] == self.required_state.0[light];
        let possible_buttons = self.valid_buttons_for_light(light, &state.buttons.0);

        let mut smallest_found = None;

        let set_sizes = if matches_required_state {
            (0..=possible_buttons.len()).step_by(2)
        } else {
            (1..=possible_buttons.len()).step_by(2)
        };
        for pressed_button_set in
            set_sizes.flat_map(|k| possible_buttons.iter().copied().combinations(k))
        {
            // pressing more buttons won't do anything
            if smallest_found.is_some_and(|smallest_found| {
                state.num_pressed + pressed_button_set.len() >= smallest_found
            }) {
                break;
            }

            let mut new_state = state.clone();

            for button_index in pressed_button_set.iter().copied() {
                let button = &self.buttons[button_index];
                new_state.press_button(button_index, button);
            }

            if let Some(count) = new_state.is_solution(&self.required_state) {
                if count == 1 {
                    return Some(1);
                }

                smallest_found = smallest_found.map(|s| s.min(count)).or(Some(count));

                continue;
            }

            if light >= self.required_state.0.len() {
                continue;
            }

            new_state.num_pressed += pressed_button_set.len();
            for excluded_button_index in possible_buttons
                .iter()
                .filter(|b| !pressed_button_set.contains(b))
            {
                new_state.buttons.0[*excluded_button_index] = ButtonState::Unpressed;
            }

            // check the next light with the new state
            if let Some(count) = self.smallest_button_set_from_light(light + 1, new_state) {
                smallest_found = smallest_found.map(|s| s.min(count)).or(Some(count));
            }
        }

        smallest_found
    }

    fn valid_buttons_for_light(&self, light: usize, button_states: &[ButtonState]) -> Vec<usize> {
        self.light_button_map
            .buttons_for_light(light)
            .iter()
            .copied()
            .filter(|b| button_states[*b] == ButtonState::Unknown)
            .collect()
    }
}

#[derive(Clone, Debug)]
struct AlgState {
    lights: Lights,
    buttons: ButtonList,
    num_pressed: usize,
}

impl AlgState {
    fn new(light_count: usize, buttons: &[Button]) -> Self {
        let lights = Lights::new(light_count);
        let buttons = ButtonList::new(buttons.len());

        Self {
            lights,
            buttons,
            num_pressed: 0,
        }
    }

    fn press_button(&mut self, idx: usize, button: &Button) {
        self.lights.push_button(button);
        self.buttons.0[idx] = ButtonState::Pressed;
    }

    fn is_solution(&self, required_state: &Lights) -> Option<usize> {
        if self.lights == *required_state {
            Some(
                self.buttons
                    .0
                    .iter()
                    .filter(|b| **b == ButtonState::Pressed)
                    .count(),
            )
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum ButtonState {
    Pressed,
    Unpressed,
    #[default]
    Unknown,
}

#[derive(Clone, Debug)]
struct ButtonList(ArrayVec<ButtonState, MAX_BUTTONS>);

impl ButtonList {
    fn new(button_count: usize) -> Self {
        let mut state = ArrayVec::new();
        state.extend(std::iter::repeat_n(ButtonState::default(), button_count));
        Self(state)
    }
}

#[derive(Clone, Debug)]
struct LightToButtonMap(ArrayVec<ArrayVec<usize, MAX_BUTTONS>, MAX_LIGHTS>);

impl LightToButtonMap {
    fn from_buttons(light_count: usize, buttons: &[Button]) -> Self {
        let mut map = ArrayVec::new();
        map.extend(std::iter::repeat_n(ArrayVec::default(), light_count));

        for (i, b) in buttons.iter().enumerate() {
            map[b.0[0]].push(i);
        }

        Self(map)
    }

    fn buttons_for_light(&self, light: usize) -> &[usize] {
        &self.0[light]
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
            .map(Machine::smallest_button_set)
            .sum::<usize>()
            .to_string()
    }
}
