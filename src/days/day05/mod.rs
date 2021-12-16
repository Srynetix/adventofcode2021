//! # Day 5: Hydrothermal Venture
//!
//! You come across a field of hydrothermal vents on the ocean floor! These vents constantly produce large, opaque clouds, so it would be best to avoid them if possible.
//!
//! They tend to form in lines; the submarine helpfully produces a list of nearby lines of vents (your puzzle input) for you to review. For example:
//!
//! 0,9 -> 5,9
//! 8,0 -> 0,8
//! 9,4 -> 3,4
//! 2,2 -> 2,1
//! 7,0 -> 7,4
//! 6,4 -> 2,0
//! 0,9 -> 2,9
//! 3,4 -> 1,4
//! 0,0 -> 8,8
//! 5,5 -> 8,2
//!
//! Each line of vents is given as a line segment in the format x1,y1 -> x2,y2 where x1,y1 are the coordinates of one end the line segment and x2,y2 are the coordinates of the other end. These line segments include the points at both ends. In other words:
//!
//! An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
//! An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.
//! For now, only consider horizontal and vertical lines: lines where either x1 = x2 or y1 = y2.
//!
//! So, the horizontal and vertical lines from the above list would produce the following diagram:
//!
//! .......1..
//! ..1....1..
//! ..1....1..
//! .......1..
//! .112111211
//! ..........
//! ..........
//! ..........
//! ..........
//! 222111....
//!
//! In this diagram, the top left corner is 0,0 and the bottom right corner is 9,9. Each position is shown as the number of lines which cover that point or . if no line covers that point. The top-left pair of 1s, for example, comes from 2,2 -> 2,1; the very bottom row is formed by the overlapping lines 0,9 -> 5,9 and 0,9 -> 2,9.
//!
//! To avoid the most dangerous areas, you need to determine the number of points where at least two lines overlap. In the above example, this is anywhere in the diagram with a 2 or larger - a total of 5 points.
//!
//! Consider only horizontal and vertical lines. At how many points do at least two lines overlap?
//!
//! Your puzzle answer was 4993.
//!
//! ## Part Two
//!
//! Unfortunately, considering only horizontal and vertical lines doesn't give you the full picture; you need to also consider diagonal lines.
//!
//! Because of the limits of the hydrothermal vent mapping system, the lines in your list will only ever be horizontal, vertical, or a diagonal line at exactly 45 degrees. In other words:
//!
//! An entry like 1,1 -> 3,3 covers points 1,1, 2,2, and 3,3.
//! An entry like 9,7 -> 7,9 covers points 9,7, 8,8, and 7,9.
//!
//! Considering all lines from the above example would now produce the following diagram:
//!
//! 1.1....11.
//! .111...2..
//! ..2.1.111.
//! ...1.2.2..
//! .112313211
//! ...1.2....
//! ..1...1...
//! .1.....1..
//! 1.......1.
//! 222111....
//!
//! You still need to determine the number of points where at least two lines overlap. In the above example, this is still anywhere in the diagram with a 2 or larger - now a total of 12 points.
//!
//! Consider all of the lines. At how many points do at least two lines overlap?
//!
//! Your puzzle answer was 21101.

use std::{cmp::Ordering, collections::HashMap};

use crate::{day::Challenge, parse_input_str};

/// Day 05 implementation.
pub struct Day05;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct PointI32 {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct LineI32 {
    a: PointI32,
    b: PointI32,
}

struct LineParser;
struct LineCollisioner;

impl PointI32 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl LineI32 {
    fn cmp_i32(a: i32, b: i32) -> i32 {
        match a.cmp(&b) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        }
    }

    pub fn iter_points(&self) -> Vec<PointI32> {
        let mut points = Vec::new();

        let x_dir = Self::cmp_i32(self.a.x, self.b.x);
        let y_dir = Self::cmp_i32(self.a.y, self.b.y);
        let len = (self.a.x - self.b.x).abs().max((self.a.y - self.b.y).abs());

        for count in 0..=len {
            let x = self.a.x + count * x_dir;
            let y = self.a.y + count * y_dir;
            points.push(PointI32::new(x, y));
        }

        points
    }
}

impl From<&str> for LineI32 {
    fn from(s: &str) -> Self {
        let vecs = s
            .split(" -> ")
            .map(|n| {
                let nums = n
                    .split(',')
                    .flat_map(|x| x.parse::<i32>().ok())
                    .collect::<Vec<i32>>();

                PointI32::new(nums[0], nums[1])
            })
            .collect::<Vec<PointI32>>();

        Self {
            a: vecs[0],
            b: vecs[1],
        }
    }
}

impl LineParser {
    pub fn parse_lines(input: &[&str]) -> Vec<LineI32> {
        input.iter().map(|&x| x.into()).collect()
    }
}

impl LineCollisioner {
    pub fn filter_horizontal_and_vertical(lines: &[LineI32]) -> Vec<LineI32> {
        lines
            .iter()
            .filter(|&l| l.a.x == l.b.x || l.a.y == l.b.y)
            .cloned()
            .collect()
    }

    pub fn scan_line_intersections(lines: &[LineI32]) -> HashMap<PointI32, u32> {
        let mut intersections = HashMap::new();

        for line in lines {
            for point in line.iter_points() {
                *intersections.entry(point).or_insert(0) += 1;
            }
        }

        intersections
    }

    pub fn count_overlaps(map: HashMap<PointI32, u32>) -> u32 {
        map.iter().filter(|(_k, &v)| v >= 2).count() as u32
    }
}

impl Challenge for Day05 {
    fn run_ex1(&mut self) -> String {
        let lines = LineParser::parse_lines(&parse_input_str!());
        let lines = LineCollisioner::filter_horizontal_and_vertical(&lines);
        let collisions = LineCollisioner::scan_line_intersections(&lines);
        LineCollisioner::count_overlaps(collisions).to_string()
    }

    fn run_ex2(&mut self) -> String {
        let lines = LineParser::parse_lines(&parse_input_str!());
        let collisions = LineCollisioner::scan_line_intersections(&lines);
        LineCollisioner::count_overlaps(collisions).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::create_day_tests;

    use super::{LineCollisioner, LineParser};

    create_day_tests!("05", "4993", "21101");

    const SAMPLE_DATA: &[&str] = &[
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    ];

    #[test]
    fn test_sample() {
        let lines = LineParser::parse_lines(SAMPLE_DATA);
        let lines = LineCollisioner::filter_horizontal_and_vertical(&lines);
        let collisions = LineCollisioner::scan_line_intersections(&lines);
        assert_eq!(LineCollisioner::count_overlaps(collisions), 5);
    }

    #[test]
    fn test_sample_diagonal() {
        let lines = LineParser::parse_lines(SAMPLE_DATA);
        let collisions = LineCollisioner::scan_line_intersections(&lines);
        assert_eq!(LineCollisioner::count_overlaps(collisions), 12);
    }
}
