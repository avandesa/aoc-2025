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

    fn max_12_joltage(&self) -> u64 {
        // Initialize with the first 12 elements
        let mut maxes: Vec<_> = self.0[0..12].into();

        for next in self.0.iter().skip(12).copied() {
            // If any element is smaller than the one to its right, pop it out and replace it with
            // the next in the bank
            if let Some(i) = maxes
                .windows(2)
                .enumerate()
                .find_map(|(i, w)| (w[0] < w[1]).then_some(i))
            {
                maxes.remove(i);
                maxes.push(next);
            } else if *maxes.last().unwrap() < next {
                maxes.pop();
                maxes.push(next);
            }
        }

        maxes
            .iter()
            .rev()
            .enumerate()
            .map(|(pow, d)| 10u64.pow(pow as u32) * *d as u64)
            .sum()
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

    fn part2(&self) -> String {
        self.banks
            .iter()
            .map(Bank::max_12_joltage)
            .sum::<u64>()
            .to_string()
    }
}
