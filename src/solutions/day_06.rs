use itertools::Itertools;

use crate::solution::Solution;

#[derive(Clone, Debug)]
enum Problem {
    Add(Vec<u32>),
    Mul(Vec<u32>),
}

impl Problem {
    fn eval(&self) -> u64 {
        match self {
            Self::Add(v) => v.iter().map(|v| *v as u64).sum(),
            Self::Mul(v) => v.iter().copied().fold(1, |acc, v| acc * v as u64),
        }
    }
}

#[derive(Debug)]
pub struct Day06 {
    lines: Vec<Vec<u8>>,
}

fn bytes_to_u32(digits: &[u8]) -> u32 {
    let pow = (digits.len() - 1) as u32;
    digits
        .iter()
        .copied()
        .enumerate()
        .map(|(i, b)| {
            debug_assert!(b.is_ascii_digit());
            let d = (b - b'0') as u32;
            10u32.pow(pow - i as u32) * d
        })
        .sum()
}

impl Day06 {
    fn parse_input_part1(&self) -> Vec<Problem> {
        let mut problem_inputs = Vec::<Vec<u32>>::new();
        for line in &self.lines {
            if matches!(line[0], b'+' | b'*') {
                return problem_inputs
                    .into_iter()
                    .zip(line.split(|b| *b == b' ').filter(|slice| !slice.is_empty()))
                    .map(|(inputs, op)| match op[0] {
                        b'+' => Problem::Add(inputs),
                        b'*' => Problem::Mul(inputs),
                        _ => panic!("invalid op"),
                    })
                    .collect();
            }

            for (i, n) in line
                .split(|b| *b == b' ')
                .filter(|slice| !slice.is_empty())
                .enumerate()
            {
                if i >= problem_inputs.len() {
                    problem_inputs.push(Vec::new())
                }

                problem_inputs[i].push(bytes_to_u32(n))
            }
        }

        panic!("no op line found");
    }

    fn parse_input_part2(&self) -> Vec<Problem> {
        let mut problems = Vec::new();

        let mut current_problem = Vec::new();
        let mut digits = Vec::new();
        let mut current_op = None;
        for col in 0..self.lines[0].len() {
            digits.clear();
            digits.extend((0..self.lines.len() - 1).filter_map(|row| {
                let b = self.lines[row][col];
                if b.is_ascii_digit() { Some(b) } else { None }
            }));

            if digits.is_empty() {
                let problem = match current_op.take().unwrap() {
                    b'+' => Problem::Add(current_problem.clone()),
                    b'*' => Problem::Mul(current_problem.clone()),
                    _ => panic!(),
                };
                problems.push(problem);
                current_problem.clear();

                continue;
            }

            current_problem.push(bytes_to_u32(&digits));

            let last = self.lines.last().unwrap()[col];
            if matches!(last, b'+' | b'*') {
                current_op = Some(last);
            }
        }

        let problem = match current_op.unwrap() {
            b'+' => Problem::Add(current_problem.clone()),
            b'*' => Problem::Mul(current_problem.clone()),
            _ => panic!(),
        };
        problems.push(problem);

        problems
    }
}

impl Solution for Day06 {
    fn with_input(input: String) -> Self {
        let lines = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.bytes().collect_vec())
            .collect();
        Self { lines }
    }

    fn part1(&self) -> String {
        self.parse_input_part1()
            .iter()
            .map(Problem::eval)
            .sum::<u64>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.parse_input_part2()
            .iter()
            .map(Problem::eval)
            .sum::<u64>()
            .to_string()
    }
}
