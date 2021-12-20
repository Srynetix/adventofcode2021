//! # Day 9: Smoke Basin
//!
//! These caves seem to be lava tubes. Parts are even still volcanically active; small hydrothermal vents release smoke into the caves that slowly settles like rain.
//!
//! If you can model how the smoke flows through the caves, you might be able to avoid it and be that much safer. The submarine generates a heightmap of the floor of the nearby caves for you (your puzzle input).
//!
//! Smoke flows to the lowest point of the area it's in. For example, consider the following heightmap:
//!
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! Each number corresponds to the height of a particular location, where 9 is the highest and 0 is the lowest a location can be.
//!
//! Your first goal is to find the low points - the locations that are lower than any of its adjacent locations. Most locations have four adjacent locations (up, down, left, and right); locations on the edge or corner of the map have three or two adjacent locations, respectively. (Diagonal locations do not count as adjacent.)
//!
//! In the above example, there are four low points, all highlighted: two are in the first row (a 1 and a 0), one is in the third row (a 5), and one is in the bottom row (also a 5). All other locations on the heightmap have some lower adjacent location, and so are not low points.
//!
//! The risk level of a low point is 1 plus its height. In the above example, the risk levels of the low points are 2, 1, 6, and 6. The sum of the risk levels of all low points in the heightmap is therefore 15.
//!
//! Find all of the low points on your heightmap. What is the sum of the risk levels of all low points on your heightmap?
//!
//! Your puzzle answer was 631.
//!
//! ## Part Two
//!
//! Next, you need to find the largest basins so you know what areas are most important to avoid.
//!
//! A basin is all locations that eventually flow downward to a single low point. Therefore, every low point has a basin, although some basins are very small. Locations of height 9 do not count as being in any basin, and all other locations will always be part of exactly one basin.
//!
//! The size of a basin is the number of locations within the basin, including the low point. The example above has four basins.
//!
//! The top-left basin, size 3:
//!
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//!
//! The top-right basin, size 9:
//!
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//!
//! The middle basin, size 14:
//!
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//!
//! The bottom-right basin, size 9:
//!
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//!
//! Find the three largest basins and multiply their sizes together. In the above example, this is 9 * 14 * 9 = 1134.
//!
//! What do you get if you multiply together the sizes of the three largest basins?
//!
//! Your puzzle answer was 821560.

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use image::ImageBuffer;
use itertools::Itertools;

use crate::{day::Challenge, parse_input_raw};

/// Day 09 implementation.
pub struct Day09;

struct HeightMap {
    width: usize,
    height: usize,
    data: Vec<u32>,
}

fn state_to_color(state: u32) -> &'static [u8; 3] {
    // Palette from https://coolors.co/d5bff2-d3bae3-d1b6d6-b197bd-706791-4e507a-364066-1d3057-10274f-031e47
    match state {
        9 => &[213, 191, 242],
        8 => &[211, 186, 227],
        7 => &[209, 182, 214],
        6 => &[177, 151, 189],
        5 => &[112, 103, 145],
        4 => &[78, 80, 122],
        3 => &[54, 64, 102],
        2 => &[29, 48, 87],
        1 => &[16, 39, 79],
        0 => &[3, 30, 71],
        _ => unreachable!(),
    }
}

fn save_heightmap_to_disk(map: &HeightMap, output: &Path) {
    let mut buffer = ImageBuffer::new(map.width as u32, map.height as u32);

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let pos = map.xy_to_pos(x as usize, y as usize);
        let state = map.data[pos];
        *pixel = image::Rgb(*state_to_color(state));
    }

    buffer.save(output).expect("Could not generate heightmap.");
}

impl HeightMap {
    pub fn find_low_points(&self) -> Vec<u32> {
        self.find_low_point_positions()
            .iter()
            .map(|&x| self.data[x])
            .collect()
    }

    pub fn find_low_point_positions(&self) -> Vec<usize> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(idx, _x)| {
                if self.value_lower_than_grid_neighbors(idx) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn find_basin_positions(&self) -> Vec<Vec<usize>> {
        let positions = self.find_low_point_positions();
        positions
            .iter()
            .map(|&x| self.discover_basin_at_position(x))
            .collect()
    }

    pub fn largest_basin_mul(&self, basins: Vec<Vec<usize>>) -> usize {
        let sorted: Vec<_> = basins.iter().map(|x| x.len()).sorted().rev().collect();
        sorted.iter().take(3).product()
    }

    fn discover_basin_at_position(&self, position: usize) -> Vec<usize> {
        let mut set = HashSet::new();
        set.insert(position);
        self.discover_basin_at_position_rec(position, &mut set);
        set.into_iter().sorted().collect()
    }

    fn discover_basin_at_position_rec(&self, position: usize, found: &mut HashSet<usize>) {
        for npos in self.grid_neighbors_position(position) {
            if found.contains(&npos) {
                continue;
            }

            let other_value = self.data[npos];
            if other_value < 9 {
                found.insert(npos);
                self.discover_basin_at_position_rec(npos, found);
            }
        }
    }

    pub fn compute_risk_level_sum(&self, points: &[u32]) -> u32 {
        points.iter().map(|x| x + 1).sum()
    }

    fn value_lower_than_grid_neighbors(&self, position: usize) -> bool {
        let this_value = self.data[position];
        for npos in self.grid_neighbors_position(position) {
            let value = self.data[npos];
            if value <= this_value {
                return false;
            }
        }

        true
    }

    #[inline]
    fn pos_to_xy(&self, position: usize) -> (usize, usize) {
        (
            position % self.width,
            (position as f32 / self.width as f32) as usize,
        )
    }

    #[inline]
    fn xy_to_pos(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    fn grid_neighbors_position(&self, position: usize) -> Vec<usize> {
        let mut neighbors_pos = Vec::new();
        let (x, y) = self.pos_to_xy(position);

        for ny in -1..=1 {
            for nx in -1..=1 {
                let cx = x as i32 + nx;
                let cy = y as i32 + ny;

                if cx < 0
                    || cy < 0
                    || cx >= self.width as i32
                    || cy >= self.height as i32
                    || (nx == 0 && ny == 0)
                    || (nx == ny)
                    || (nx == -ny)
                    || (-nx == ny)
                {
                    continue;
                } else {
                    let pos = self.xy_to_pos(cx as usize, cy as usize);
                    neighbors_pos.push(pos);
                }
            }
        }

        neighbors_pos
    }
}

impl From<&str> for HeightMap {
    fn from(s: &str) -> Self {
        let lines = s.lines().collect::<Vec<_>>();
        let height = lines.len();
        let width = lines[0].len();
        let mut data = Vec::with_capacity(width * height);

        for line in lines {
            for c in line.chars() {
                data.push(c.to_digit(10).expect("Char should be a digit"));
            }
        }

        Self {
            width,
            height,
            data,
        }
    }
}

impl Challenge for Day09 {
    fn run_ex1(&mut self) -> String {
        let hm = HeightMap::from(parse_input_raw!());
        let path = PathBuf::new().join("debug").join("day09.png");
        save_heightmap_to_disk(&hm, &path);

        let points = hm.find_low_points();
        hm.compute_risk_level_sum(&points).to_string()
    }

    fn run_ex2(&mut self) -> String {
        let hm = HeightMap::from(parse_input_raw!());
        let basins = hm.find_basin_positions();
        hm.largest_basin_mul(basins).to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::create_day_tests;

    use super::{save_heightmap_to_disk, HeightMap};

    create_day_tests!("09", "631", "821560");

    const SAMPLE_DATA: &str = indoc::indoc! {"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    "};

    #[test]
    fn test_sample() {
        let hm = HeightMap::from(SAMPLE_DATA);
        let path = PathBuf::new().join("debug").join("day09-sample.png");
        save_heightmap_to_disk(&hm, &path);
        assert_eq!(hm.find_low_points(), &[1, 0, 5, 5]);
    }

    #[test]
    fn test_sample_sum() {
        let hm = HeightMap::from(SAMPLE_DATA);
        assert_eq!(hm.compute_risk_level_sum(&[1, 0, 5, 5]), 15);
    }

    #[test]
    fn test_discover_basin() {
        let hm = HeightMap::from(SAMPLE_DATA);
        assert_eq!(hm.discover_basin_at_position(0), &[0, 1, 10]);
        assert_eq!(
            hm.discover_basin_at_position(9),
            &[5, 6, 7, 8, 9, 16, 18, 19, 29]
        );
    }

    #[test]
    fn test_sample_basin() {
        let hm = HeightMap::from(SAMPLE_DATA);
        let basins = hm.find_basin_positions();
        assert_eq!(hm.largest_basin_mul(basins), 1134);
    }
}
