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

/// Parse input data as string.
#[macro_export]
macro_rules! parse_input_str {
    () => {{
        let input_data: &str = include_str!("input.txt");
        input_data.lines().collect::<Vec<_>>()
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

            let (d1_output, d1_elapsed) = {
                let now = std::time::Instant::now();
                let output = day.run_ex1();
                (output, now.elapsed().as_millis().to_string())
            };
            let (d2_output, d2_elapsed) = {
                let now = std::time::Instant::now();
                let output = day.run_ex2();
                (output, now.elapsed().as_millis().to_string())
            };

            println!(
                "📆 {} : 🌓 {} ({}ms) — 🌕 {} ({}ms)",
                $day.bold().blue(), d1_output.green(), d1_elapsed.yellow(), d2_output.green(), d2_elapsed.yellow()
            );
        }
    }};
}

/// Create day tests.
#[macro_export]
macro_rules! create_day_tests {
    ($day:literal, $ans1:literal, $ans2:literal) => {
        paste::paste! {
            use $crate::days::[<day $day>]::[<Day $day>];
            use $crate::day::Challenge;

            #[test]
            fn test_ex1() {
                let mut day = [<Day $day>]::new();
                assert_eq!(day.run_ex1(), $ans1);
            }

            #[test]
            fn test_ex2() {
                let mut day = [<Day $day>]::new();
                assert_eq!(day.run_ex2(), $ans2);
            }
        }
    };
}
