use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use clap::Parser;
use color_eyre::eyre::{Context, Result, bail};

use crate::solution::Solution;

mod solution;
mod solutions;

const INPUT_DIR: &str = "inputs";

macro_rules! solver {
    ($day:ident, $raw_input:ident) => {{
        let (solver, elapsed) = time(|| solutions::$day::with_input($raw_input));
        (Box::new(solver) as Box<dyn Solution>, elapsed)
    }};
}

#[derive(Debug, Clone, Parser)]
pub struct Args {
    day: u32,
    #[clap(long)]
    example: Option<String>,
    #[clap(long)]
    example_path: Option<PathBuf>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let raw_input = if let Some(example_path) = args.example_path {
        std::fs::read_to_string(&example_path).wrap_err_with(|| {
            format!("example file does not exist: `{}`", example_path.display())
        })?
    } else if let Some(example) = args.example {
        example
    } else {
        let input_path = PathBuf::from(INPUT_DIR).join(format!("day_{:02}.txt", args.day));
        std::fs::read_to_string(&input_path)
            .wrap_err_with(|| format!("example file does not exist: `{}`", input_path.display()))?
    };

    let (solver, input_build_time) = match args.day {
        1 => solver!(Day01, raw_input),
        2 => solver!(Day02, raw_input),
        3 => solver!(Day03, raw_input),
        4 => solver!(Day04, raw_input),
        5 => solver!(Day05, raw_input),
        6 => solver!(Day06, raw_input),
        7 => solver!(Day07, raw_input),
        8 => solver!(Day08, raw_input),
        _ => bail!("invalid day: `{}`", args.day),
    };
    println!("Parsed input int {input_build_time:#?}");

    let (part1, elapsed) = time(|| solver.part1());
    println!("Day {} Part 1:", args.day);
    println!("{part1}");
    println!("Took {elapsed:#?}");

    println!();

    let (part2, elapsed) = time(|| solver.part2());
    println!("Day {} Part 2:", args.day);
    println!("{part2}");
    println!("Took {elapsed:#?}");

    Ok(())
}

fn time<T, F: FnOnce() -> T>(f: F) -> (T, Duration) {
    let start = Instant::now();
    let input = f();
    (input, start.elapsed())
}
