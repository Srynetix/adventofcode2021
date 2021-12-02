//! # Day 2: Dive!
//!
//! Now, you need to figure out how to pilot this thing.
//!
//! It seems like the submarine can take a series of commands like forward 1, down 2, or up 3:
//!
//! forward X increases the horizontal position by X units.
//! down X increases the depth by X units.
//! up X decreases the depth by X units.
//! Note that since you're on a submarine, down and up affect your depth, and so they have the opposite result of what you might expect.
//!
//! The submarine seems to already have a planned course (your puzzle input). You should probably figure out where it's going. For example:
//!
//! ```text
//! forward 5
//! down 5
//! forward 8
//! up 3
//! down 8
//! forward 2
//! ```
//!
//! Your horizontal position and depth both start at 0. The steps above would then modify them as follows:
//!
//! - forward 5 adds 5 to your horizontal position, a total of 5.
//! - down 5 adds 5 to your depth, resulting in a value of 5.
//! - forward 8 adds 8 to your horizontal position, a total of 13.
//! - up 3 decreases your depth by 3, resulting in a value of 2.
//! - down 8 adds 8 to your depth, resulting in a value of 10.
//! - forward 2 adds 2 to your horizontal position, a total of 15.
//!
//! After following these instructions, you would have a horizontal position of 15 and a depth of 10. (Multiplying these together produces 150.)
//!
//! Calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?
//!
//! Your puzzle answer was 2102357.
//!
//! ## Part Two
//!
//! Based on your calculations, the planned course doesn't seem to make any sense. You find the submarine manual and discover that the process is actually slightly more complicated.
//!
//! In addition to horizontal position and depth, you'll also need to track a third value, aim, which also starts at 0. The commands also mean something entirely different than you first thought:
//!
//! - down X increases your aim by X units.
//! - up X decreases your aim by X units.
//! - forward X does two things:
//!     - It increases your horizontal position by X units.
//!     - It increases your depth by your aim multiplied by X.
//!
//! Again note that since you're on a submarine, down and up do the opposite of what you might expect: "down" means aiming in the positive direction.
//!
//! Now, the above example does something different:
//!
//! - forward 5 adds 5 to your horizontal position, a total of 5. Because your aim is 0, your depth does not change.
//! - down 5 adds 5 to your aim, resulting in a value of 5.
//! - forward 8 adds 8 to your horizontal position, a total of 13. Because your aim is 5, your depth increases by 8*5=40.
//! - up 3 decreases your aim by 3, resulting in a value of 2.
//! - down 8 adds 8 to your aim, resulting in a value of 10.
//! - forward 2 adds 2 to your horizontal position, a total of 15. Because your aim is 10, your depth increases by 2*10=20 to a total of 60.
//!
//! After following these new instructions, you would have a horizontal position of 15 and a depth of 60. (Multiplying these produces 900.)
//!
//! Using this new interpretation of the commands, calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?
//! 
//! Your puzzle answer was 2101031224.

use crate::{day::Challenge, parse_input_str};

/// Day 02 implementation.
pub struct Day02;

struct Submarine {
    depth: i32,
    horizontal: i32,
    aim: Option<i32>,
}

#[derive(Debug, PartialEq)]
enum MoveDirection {
    Up,
    Down,
    Forward,
}

#[derive(Debug, PartialEq)]
struct Move {
    direction: MoveDirection,
    amount: i32,
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        let mut split = value.split_whitespace();
        let direction = split
            .next()
            .expect("First parameter should be a valid direction")
            .into();
        let amount = split
            .next()
            .and_then(|x| x.parse().ok())
            .expect("Second parameter should be a valid integer");

        Self { direction, amount }
    }
}

impl From<&str> for MoveDirection {
    fn from(value: &str) -> Self {
        match value {
            "forward" => Self::Forward,
            "up" => Self::Up,
            "down" => Self::Down,
            _ => panic!("Unknown direction '{}'.", value),
        }
    }
}

impl Submarine {
    pub fn new() -> Self {
        Self {
            depth: 0,
            horizontal: 0,
            aim: None,
        }
    }

    pub fn new_with_aim() -> Self {
        Self {
            depth: 0,
            horizontal: 0,
            aim: Some(0),
        }
    }

    pub fn from_moves(input: &[&str]) -> Self {
        let mut sub = Self::new();
        sub.execute_moves(&Self::parse_moves(input));
        sub
    }

    pub fn from_moves_with_aim(input: &[&str]) -> Self {
        let mut sub = Self::new_with_aim();
        sub.execute_moves(&Self::parse_moves(input));
        sub
    }

    pub fn parse_moves(input: &[&str]) -> Vec<Move> {
        input.iter().map(|&d| d.into()).collect()
    }

    pub fn execute_moves(&mut self, moves: &[Move]) {
        for mv in moves {
            match self.aim {
                None => match mv.direction {
                    MoveDirection::Up => self.depth -= mv.amount,
                    MoveDirection::Down => self.depth += mv.amount,
                    MoveDirection::Forward => self.horizontal += mv.amount,
                },
                Some(a) => match mv.direction {
                    MoveDirection::Up => self.aim = Some(a - mv.amount),
                    MoveDirection::Down => self.aim = Some(a + mv.amount),
                    MoveDirection::Forward => {
                        self.horizontal += mv.amount;
                        self.depth += a * mv.amount;
                    }
                },
            }
        }
    }

    pub fn score(&self) -> i32 {
        self.depth * self.horizontal
    }
}

impl Challenge for Day02 {
    fn new() -> Self {
        Self
    }

    fn run_ex1(&mut self) -> String {
        Submarine::from_moves(&parse_input_str!())
            .score()
            .to_string()
    }

    fn run_ex2(&mut self) -> String {
        Submarine::from_moves_with_aim(&parse_input_str!())
            .score()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::create_day_tests;

    use super::{Move, MoveDirection, Submarine};

    const SAMPLE_DATA: [&str; 6] = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    #[test]
    fn test_parse_move() {
        let moves: Vec<Move> = Submarine::parse_moves(&SAMPLE_DATA);
        assert_eq!(
            moves,
            vec![
                Move {
                    direction: MoveDirection::Forward,
                    amount: 5
                },
                Move {
                    direction: MoveDirection::Down,
                    amount: 5
                },
                Move {
                    direction: MoveDirection::Forward,
                    amount: 8
                },
                Move {
                    direction: MoveDirection::Up,
                    amount: 3
                },
                Move {
                    direction: MoveDirection::Down,
                    amount: 8
                },
                Move {
                    direction: MoveDirection::Forward,
                    amount: 2
                },
            ]
        )
    }

    #[test]
    fn test_execute_moves() {
        let sub = Submarine::from_moves(&SAMPLE_DATA);
        assert_eq!((sub.horizontal, sub.depth, sub.score()), (15, 10, 150));
    }

    #[test]
    fn test_execute_moves_with_aim() {
        let sub = Submarine::from_moves_with_aim(&SAMPLE_DATA);
        assert_eq!((sub.horizontal, sub.depth, sub.score()), (15, 60, 900));
    }

    create_day_tests!("02", "2102357", "2101031224");
}
