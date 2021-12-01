//! Day common utils.

use std::{fmt::Debug, str::FromStr};

/// Parse lines from input.
pub fn parse_lines<T>(data: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    data.lines()
        .map(|x| x.parse().expect("Could not parse lines from input data."))
        .collect()
}

/// Challenge.
pub trait Challenge {
    /// New challenge.
    fn new() -> Self;
    /// Exercise 1.
    fn run_ex1(&mut self) -> String;
    /// Exercise 2.
    fn run_ex2(&mut self) -> String;
}

/// Parse input data.
#[macro_export]
macro_rules! parse_input {
    () => {{
        let input_data: &str = include_str!("input.txt");
        $crate::day::parse_lines(input_data)
    }};
}

/// Run day.
#[macro_export]
macro_rules! run_day {
    ($day:literal) => {{
        paste::paste! {
            use $crate::days::[<day $day>]::[<Day $day>];
            use owo_colors::OwoColorize;

            let mut day = [<Day $day>]::new();
            println!("📆 {} : 🌓 {} — 🌕 {}", $day.bold().blue(), day.run_ex1().green(), day.run_ex2().green());
        }
    }};
}
