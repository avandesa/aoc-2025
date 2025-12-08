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
    input: String,
}

impl Day06 {
    fn parse_input_part1(&self) -> Vec<Problem> {
        let mut problem_inputs = Vec::<Vec<u32>>::new();
        for line in self.input.lines() {
            if matches!(line.chars().next().unwrap(), '+' | '*') {
                let problems = problem_inputs
                    .into_iter()
                    .zip(line.split_whitespace())
                    .map(|(inputs, op)| match op {
                        "+" => Problem::Add(inputs),
                        "*" => Problem::Mul(inputs),
                        _ => panic!("invalid op"),
                    })
                    .collect();

                return problems;
            }

            for (i, n) in line.split_whitespace().enumerate() {
                if i >= problem_inputs.len() {
                    problem_inputs.push(Vec::new())
                }

                let n = n.parse().unwrap();
                problem_inputs[i].push(n)
            }
        }

        panic!("no op line found");
    }

    fn parse_input_part2(&self) -> Vec<Problem> {
        let lines = self
            .input
            .lines()
            .filter_map(|l| {
                if l.is_empty() {
                    None
                } else {
                    Some(l.bytes().collect_vec())
                }
            })
            .collect_vec();

        let mut problems = Vec::new();

        let mut current_problem = Vec::new();
        let mut digits = Vec::new();
        let mut current_op = None;
        for col in 0..lines[0].len() {
            digits.clear();
            digits.extend((0..lines.len() - 1).filter_map(|row| {
                let b = lines[row][col];
                if b.is_ascii_digit() {
                    let d = b - b'0';
                    Some(d as u32)
                } else {
                    None
                }
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

            let pow = (digits.len() - 1) as u32;
            let current_number = digits
                .iter()
                .copied()
                .enumerate()
                .map(|(i, d)| 10u32.pow(pow - i as u32) * d)
                .sum();
            current_problem.push(current_number);

            let last = lines.last().unwrap()[col];
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
        Self { input }
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
