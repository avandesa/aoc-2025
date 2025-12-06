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
    problems: Vec<Problem>,
}

impl Solution for Day06 {
    fn with_input(input: String) -> Self {
        let mut problem_inputs = Vec::<Vec<u32>>::new();
        for line in input.lines() {
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

                return Self { problems };
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

    fn part1(&self) -> String {
        self.problems
            .iter()
            .map(Problem::eval)
            .sum::<u64>()
            .to_string()
    }
}
