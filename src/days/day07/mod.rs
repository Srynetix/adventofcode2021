//! # Day 7: The Treachery of Whales
//!
//! A giant whale has decided your submarine is its next meal, and it's much faster than you are. There's nowhere to run!
//!
//! Suddenly, a swarm of crabs (each in its own tiny submarine - it's too deep for them otherwise) zooms in to rescue you! They seem to be preparing to blast a hole in the ocean floor; sensors indicate a massive underground cave system just beyond where they're aiming!
//!
//! The crab submarines all need to be aligned before they'll have enough power to blast a large enough hole for your submarine to get through. However, it doesn't look like they'll be aligned before the whale catches you! Maybe you can help?
//!
//! There's one major catch - crab submarines can only move horizontally.
//!
//! You quickly make a list of the horizontal position of each crab (your puzzle input). Crab submarines have limited fuel, so you need to find a way to make all of their horizontal positions match while requiring them to spend as little fuel as possible.
//!
//! For example, consider the following horizontal positions:
//!
//! 16,1,2,0,4,2,7,1,2,14
//!
//! This means there's a crab with horizontal position 16, a crab with horizontal position 1, and so on.
//!
//! Each change of 1 step in horizontal position of a single crab costs 1 fuel. You could choose any horizontal position to align them all on, but the one that costs the least fuel is horizontal position 2:
//!
//! Move from 16 to 2: 14 fuel
//! Move from 1 to 2: 1 fuel
//! Move from 2 to 2: 0 fuel
//! Move from 0 to 2: 2 fuel
//! Move from 4 to 2: 2 fuel
//! Move from 2 to 2: 0 fuel
//! Move from 7 to 2: 5 fuel
//! Move from 1 to 2: 1 fuel
//! Move from 2 to 2: 0 fuel
//! Move from 14 to 2: 12 fuel
//!
//! This costs a total of 37 fuel. This is the cheapest possible outcome; more expensive outcomes include aligning at position 1 (41 fuel), position 3 (39 fuel), or position 10 (71 fuel).
//!
//! Determine the horizontal position that the crabs can align to using the least fuel possible. How much fuel must they spend to align to that position?
//!
//! Your puzzle answer was 347011.
//!
//! ## Part Two
//!
//! The crabs don't seem interested in your proposed solution. Perhaps you misunderstand crab engineering?
//!
//! As it turns out, crab submarine engines don't burn fuel at a constant rate. Instead, each change of 1 step in horizontal position costs 1 more unit of fuel than the last: the first step costs 1, the second step costs 2, the third step costs 3, and so on.
//!
//! As each crab moves, moving further becomes more expensive. This changes the best horizontal position to align them all on; in the example above, this becomes 5:
//!
//! Move from 16 to 5: 66 fuel
//! Move from 1 to 5: 10 fuel
//! Move from 2 to 5: 6 fuel
//! Move from 0 to 5: 15 fuel
//! Move from 4 to 5: 1 fuel
//! Move from 2 to 5: 6 fuel
//! Move from 7 to 5: 3 fuel
//! Move from 1 to 5: 10 fuel
//! Move from 2 to 5: 6 fuel
//! Move from 14 to 5: 45 fuel
//! This costs a total of 168 fuel. This is the new cheapest possible outcome; the old alignment position (2) now costs 206 fuel instead.
//!
//! Determine the horizontal position that the crabs can align to using the least fuel possible so they can make you an escape route! How much fuel must they spend to align to that position?
//!
//! Your puzzle answer was 98363777.

use crate::{day::Challenge, parse_input_raw};

/// Day 07 implementation.
pub struct Day07;

struct CrabSwarm {
    positions: Vec<u32>,
}

impl CrabSwarm {
    pub fn fuel_cost_for_position(&self, pos: u32) -> u32 {
        let mut sum: i32 = 0;

        for &p in &self.positions {
            let diff = if p > pos {
                p as i32 - pos as i32
            } else {
                pos as i32 - p as i32
            };

            sum += diff;
        }

        sum as u32
    }

    pub fn fuel_cost_for_position_with_sum(&self, pos: u32) -> u32 {
        let mut sum: i32 = 0;

        for &p in &self.positions {
            let diff = if p > pos {
                p as i32 - pos as i32
            } else {
                pos as i32 - p as i32
            };

            let local_sum = Self::sum(diff as usize, 1, diff) as i32;
            sum += local_sum;
        }

        sum as u32
    }

    pub fn sum(n: usize, min: i32, max: i32) -> u32 {
        ((n as f32 * (min + max) as f32) / 2.0) as u32
    }

    pub fn min_cost_for_alignment(&self) -> (u32, u32) {
        let mut min_pos = u32::MAX;
        let mut min_sum = u32::MAX;
        let max_p = *self
            .positions
            .iter()
            .max()
            .expect("Should have more than one value");
        for i in 0..max_p {
            let sum = self.fuel_cost_for_position(i);
            if sum < min_sum {
                min_sum = sum;
                min_pos = i;
            }
        }

        (min_pos, min_sum)
    }

    pub fn min_cost_for_alignment_with_sum(&self) -> (u32, u32) {
        let mut min_pos = u32::MAX;
        let mut min_sum = u32::MAX;
        let max_p = *self
            .positions
            .iter()
            .max()
            .expect("Should have more than one value");
        for i in 0..max_p {
            let sum = self.fuel_cost_for_position_with_sum(i);
            if sum < min_sum {
                min_sum = sum;
                min_pos = i;
            }
        }

        (min_pos, min_sum)
    }
}

impl From<&str> for CrabSwarm {
    fn from(s: &str) -> Self {
        let positions = s.split(',').flat_map(|x| x.parse().ok()).collect();

        Self { positions }
    }
}

impl Challenge for Day07 {
    fn new() -> Self {
        Self
    }

    fn run_ex1(&mut self) -> String {
        let swarm = CrabSwarm::from(parse_input_raw!());
        let (_pos, sum) = swarm.min_cost_for_alignment();
        sum.to_string()
    }

    fn run_ex2(&mut self) -> String {
        let swarm = CrabSwarm::from(parse_input_raw!());
        let (_pos, sum) = swarm.min_cost_for_alignment_with_sum();
        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::create_day_tests;

    use super::CrabSwarm;

    create_day_tests!("07", "347011", "98363777");

    const SAMPLE_DATA: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_sample() {
        let swarm = CrabSwarm::from(SAMPLE_DATA);
        assert_eq!(swarm.min_cost_for_alignment(), (2, 37));
    }

    #[test]
    fn test_sample_sum() {
        let swarm = CrabSwarm::from(SAMPLE_DATA);
        assert_eq!(swarm.min_cost_for_alignment_with_sum(), (5, 168));
    }
}
