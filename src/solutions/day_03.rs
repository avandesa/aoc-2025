use crate::solution::Solution;

#[derive(Debug, Clone)]
struct Bank(Vec<u8>);

impl Bank {
    fn from_line(l: &str) -> Self {
        let values = l
            .chars()
            .map(|c| {
                let val = c.to_digit(10).unwrap() as u8;
                assert!((1..=9).contains(&val));
                val
            })
            .collect();
        Self(values)
    }

    fn max_joltage(&self) -> u32 {
        let mut max_l = 0;
        let mut max_r = 0;
        // tuple_windows is slow
        for window in self.0.windows(2) {
            // casting here is faster?!
            let l = window[0] as u32;
            let r = window[1] as u32;

            if l > max_l {
                max_l = l;
                max_r = r;
            } else if r > max_r {
                max_r = r;
            }
        }

        max_l * 10 + max_r
    }
}

#[derive(Debug)]
pub struct Day03 {
    banks: Vec<Bank>,
}

impl Solution for Day03 {
    fn with_input(input: String) -> Self {
        let banks = input.trim().lines().map(Bank::from_line).collect();
        Self { banks }
    }

    fn part1(&self) -> String {
        self.banks
            .iter()
            .map(Bank::max_joltage)
            .sum::<u32>()
            .to_string()
    }
}
