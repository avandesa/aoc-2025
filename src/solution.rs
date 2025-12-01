use std::fmt::Debug;

pub trait Solution: Debug {
    fn with_input(input: String) -> Self
    where
        Self: Sized;

    fn part1(&self) -> String {
        "part 1 not solved".to_string()
    }

    fn part2(&self) -> String {
        "part 2 not solved".to_string()
    }
}
