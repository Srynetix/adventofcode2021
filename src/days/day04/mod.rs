//! # Day 4: Giant Squid
//!
//! You're already almost 1.5km (almost a mile) below the surface of the ocean, already so deep that you can't see any sunlight. What you can see, however, is a giant squid that has attached itself to the outside of your submarine.
//!
//! Maybe it wants to play bingo?
//!
//! Bingo is played on a set of boards each consisting of a 5x5 grid of numbers. Numbers are chosen at random, and the chosen number is marked on all boards on which it appears. (Numbers may not appear on all boards.) If all numbers in any row or any column of a board are marked, that board wins. (Diagonals don't count.)
//!
//! The submarine has a bingo subsystem to help passengers (currently, you and the giant squid) pass the time. It automatically generates a random order in which to draw numbers and a random set of boards (your puzzle input). For example:
//!
//! 7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
//!
//! 22 13 17 11  0
//!  8  2 23  4 24
//! 21  9 14 16  7
//!  6 10  3 18  5
//!  1 12 20 15 19
//!
//!  3 15  0  2 22
//!  9 18 13 17  5
//! 19  8  7 25 23
//! 20 11 10 24  4
//! 14 21 16 12  6
//!
//! 14 21 17 24  4
//! 10 16 15  9 19
//! 18  8 23 26 20
//! 22 11 13  6  5
//!  2  0 12  3  7
//!
//! After the first five numbers are drawn (7, 4, 9, 5, and 11), there are no winners, but the boards are marked as follows (shown here adjacent to each other to save space):
//!
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//!
//! After the next six numbers are drawn (17, 23, 2, 0, 14, and 21), there are still no winners:
//!
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//!
//! Finally, 24 is drawn:
//!
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//!
//! At this point, the third board wins because it has at least one complete row or column of marked numbers (in this case, the entire top row is marked: 14 21 17 24 4).
//!
//! The score of the winning board can now be calculated. Start by finding the sum of all unmarked numbers on that board; in this case, the sum is 188. Then, multiply that sum by the number that was just called when the board won, 24, to get the final score, 188 * 24 = 4512.
//!
//! To guarantee victory against the giant squid, figure out which board will win first. What will your final score be if you choose that board?
//!
//! Your puzzle answer was 69579.
//!
//! ## Part Two
//!
//! On the other hand, it might be wise to try a different strategy: let the giant squid win.
//!
//! You aren't sure how many bingo boards a giant squid could play at once, so rather than waste time counting its arms, the safe thing to do is to figure out which board will win last and choose that one. That way, no matter which boards it picks, it will win for sure.
//!
//! In the above example, the second board is the last to win, which happens after 13 is eventually called and its middle column is completely marked. If you were to keep playing until this point, the second board would have a sum of unmarked numbers equal to 148 for a final score of 148 * 13 = 1924.
//!
//! Figure out which board will win last. Once it wins, what would its final score be?
//!
//! Your puzzle answer was 14877.

/// Day 04 implementation.
pub struct Day04;

use crate::day::Challenge;

struct BingoParser;
struct BingoPlayer;

#[derive(Debug)]
struct BingoPlay(Vec<u32>);
#[derive(Debug)]
struct BingoGrid {
    width: usize,
    height: usize,
    data: Vec<u32>,
    found: Vec<bool>,
    won: bool,
}

impl BingoGrid {
    pub fn process_number(&mut self, number: u32) -> bool {
        if let Some(pos) = self.data.iter().position(|&x| x == number) {
            self.found[pos] = true;
            true
        } else {
            false
        }
    }

    pub fn has_won(&self) -> bool {
        for row_idx in 0..self.height {
            if self.has_complete_row(row_idx) {
                return true;
            }
        }

        for col_idx in 0..self.width {
            if self.has_complete_column(col_idx) {
                return true;
            }
        }

        false
    }

    fn has_complete_row(&self, row_idx: usize) -> bool {
        for col_idx in 0..self.width {
            let value = self.found[row_idx * self.width + col_idx];
            if !value {
                return false;
            }
        }

        true
    }

    fn has_complete_column(&self, col_idx: usize) -> bool {
        for row_idx in 0..self.height {
            let value = self.found[row_idx * self.width + col_idx];
            if !value {
                return false;
            }
        }

        true
    }

    pub fn get_unmarked_sum(&self) -> u32 {
        let mut sum = 0;
        for idx in 0..self.found.len() {
            if !self.found[idx] {
                sum += self.data[idx];
            }
        }

        sum
    }
}

impl From<&str> for BingoPlay {
    fn from(value: &str) -> Self {
        let numbers: Vec<u32> = value.split(',').flat_map(|n| n.parse().ok()).collect();
        Self(numbers)
    }
}

impl From<&[&str]> for BingoGrid {
    fn from(value: &[&str]) -> Self {
        let height = value.len();
        let grid: Vec<u32> = value
            .iter()
            .map(|n| n.split_whitespace().flat_map(|n| n.parse().ok()))
            .flatten()
            .collect();
        Self {
            width: grid.len() / height,
            height,
            found: vec![false; grid.len()],
            data: grid,
            won: false,
        }
    }
}

impl BingoParser {
    fn parse_play_and_grids(input: &str) -> (BingoPlay, Vec<BingoGrid>) {
        let data: Vec<Vec<&str>> = input.split("\n\n").map(|l| l.lines().collect()).collect();
        let play = BingoPlay::from(data[0][0]);
        let grids = data
            .iter()
            .skip(1)
            .map(|n| BingoGrid::from(n.as_ref()))
            .collect();

        (play, grids)
    }
}

impl BingoPlayer {
    fn play(play: BingoPlay, mut grids: Vec<BingoGrid>) -> u32 {
        for num in play.0 {
            for grid in grids.iter_mut() {
                grid.process_number(num);
                if grid.has_won() {
                    // Compute unmarked numbers sum, times the current number
                    return grid.get_unmarked_sum() * num;
                }
            }
        }

        panic!("No grid won.")
    }

    fn play_waiting_for_last(play: BingoPlay, mut grids: Vec<BingoGrid>) -> u32 {
        let mut wins = 0;
        let count = grids.len();
        for num in play.0 {
            for grid in grids.iter_mut() {
                if grid.won {
                    continue;
                }

                grid.process_number(num);
                if grid.has_won() {
                    grid.won = true;
                    wins += 1;

                    if wins == count {
                        // OK, last one!
                        return grid.get_unmarked_sum() * num;
                    }
                }
            }
        }

        panic!("No grid won.")
    }
}

impl Challenge for Day04 {
    fn run_ex1(&mut self) -> String {
        let input = include_str!("input.txt");
        let (play, grids) = BingoParser::parse_play_and_grids(input);
        BingoPlayer::play(play, grids).to_string()
    }

    fn run_ex2(&mut self) -> String {
        let input = include_str!("input.txt");
        let (play, grids) = BingoParser::parse_play_and_grids(input);
        BingoPlayer::play_waiting_for_last(play, grids).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::create_day_tests;

    use super::{BingoParser, BingoPlayer};

    create_day_tests!("04", "69579", "14877");

    const SAMPLE_DATA: &str = indoc::indoc! {"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
        8  2 23  4 24
        21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19

        3 15  0  2 22
        9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
        2  0 12  3  7
    "};

    #[test]
    fn test_sample() {
        let (play, grids) = BingoParser::parse_play_and_grids(SAMPLE_DATA);
        assert_eq!(BingoPlayer::play(play, grids).to_string(), "4512");
    }

    #[test]
    fn test_sample_last_one() {
        let (play, grids) = BingoParser::parse_play_and_grids(SAMPLE_DATA);
        assert_eq!(
            BingoPlayer::play_waiting_for_last(play, grids).to_string(),
            "1924"
        );
    }
}
