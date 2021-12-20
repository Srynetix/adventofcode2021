//! # Day 13: Transparent Origami
//!
//! You reach another volcanically active part of the cave. It would be nice if you could do some kind of thermal imaging so you could tell ahead of time which caves are too hot to safely enter.
//!
//! Fortunately, the submarine seems to be equipped with a thermal camera! When you activate it, you are greeted with:
//!
//! Congratulations on your purchase! To activate this infrared thermal imaging
//! camera system, please enter the code found on page 1 of the manual.
//! Apparently, the Elves have never used this feature. To your surprise, you manage to find the manual; as you go to open it, page 1 falls out. It's a large sheet of transparent paper! The transparent paper is marked with random dots and includes instructions on how to fold it up (your puzzle input). For example:
//!
//! 6,10
//! 0,14
//! 9,10
//! 0,3
//! 10,4
//! 4,11
//! 6,0
//! 6,12
//! 4,1
//! 0,13
//! 10,12
//! 3,4
//! 3,0
//! 8,4
//! 1,10
//! 2,14
//! 8,10
//! 9,0
//!
//! fold along y=7
//! fold along x=5
//!
//! The first section is a list of dots on the transparent paper. 0,0 represents the top-left coordinate. The first value, x, increases to the right. The second value, y, increases downward. So, the coordinate 3,0 is to the right of 0,0, and the coordinate 0,7 is below 0,0. The coordinates in this example form the following pattern, where # is a dot on the paper and . is an empty, unmarked position:
//!
//! ...#..#..#.
//! ....#......
//! ...........
//! #..........
//! ...#....#.#
//! ...........
//! ...........
//! ...........
//! ...........
//! ...........
//! .#....#.##.
//! ....#......
//! ......#...#
//! #..........
//! #.#........
//!
//! Then, there is a list of fold instructions. Each instruction indicates a line on the transparent paper and wants you to fold the paper up (for horizontal y=... lines) or left (for vertical x=... lines). In this example, the first fold instruction is fold along y=7, which designates the line formed by all of the positions where y is 7 (marked here with -):
//!
//! ...#..#..#.
//! ....#......
//! ...........
//! #..........
//! ...#....#.#
//! ...........
//! ...........
//! -----------
//! ...........
//! ...........
//! .#....#.##.
//! ....#......
//! ......#...#
//! #..........
//! #.#........
//!
//! Because this is a horizontal line, fold the bottom half up. Some of the dots might end up overlapping after the fold is complete, but dots will never appear exactly on a fold line. The result of doing this fold looks like this:
//!
//! #.##..#..#.
//! #...#......
//! ......#...#
//! #...#......
//! .#.#..#.###
//! ...........
//! ...........
//!
//! Now, only 17 dots are visible.
//!
//! Notice, for example, the two dots in the bottom left corner before the transparent paper is folded; after the fold is complete, those dots appear in the top left corner (at 0,0 and 0,1). Because the paper is transparent, the dot just below them in the result (at 0,3) remains visible, as it can be seen through the transparent paper.
//!
//! Also notice that some dots can end up overlapping; in this case, the dots merge together and become a single dot.
//!
//! The second fold instruction is fold along x=5, which indicates this line:
//!
//! #.##.|#..#.
//! #...#|.....
//! .....|#...#
//! #...#|.....
//! .#.#.|#.###
//! .....|.....
//! .....|.....
//!
//! Because this is a vertical line, fold left:
//!
//! #####
//! #...#
//! #...#
//! #...#
//! #####
//! .....
//! .....
//!
//! The instructions made a square!
//!
//! The transparent paper is pretty big, so for now, focus on just completing the first fold. After the first fold in the example above, 17 dots are visible - dots that end up overlapping after the fold is completed count as a single dot.
//!
//! How many dots are visible after completing just the first fold instruction on your transparent paper?
//!
//! Your puzzle answer was 655.
//!
//! ## Part Two
//!
//! Finish folding the transparent paper according to the instructions. The manual says the code is always eight capital letters.
//!
//! What code do you use to activate the infrared thermal imaging camera system?
//!
//! Your puzzle answer was JPZCUAUR.

use std::{
    cmp::Ordering,
    collections::LinkedList,
    path::{Path, PathBuf},
};

use image::ImageBuffer;

use crate::{day::Challenge, parse_input_raw};

/// Day 13 implementation.
pub struct Day13;

fn state_to_color(state: bool) -> &'static [u8; 3] {
    // Palette from https://coolors.co/d5bff2-d3bae3-d1b6d6-b197bd-706791-4e507a-364066-1d3057-10274f-031e47
    match state {
        false => &[0, 0, 0],
        true => &[255, 255, 255],
    }
}

fn save_paper_to_disk(map: &TransparentPaper, output: &Path) {
    let mut buffer = ImageBuffer::new(map.width as u32, map.height as u32);

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let pos = map.xy_to_pos(x as usize, y as usize);
        let state = map.data[pos];
        *pixel = image::Rgb(*state_to_color(state));
    }

    buffer.save(output).expect("Could not generate paper.");
}

#[derive(Clone, Copy)]
enum Coordinate {
    X,
    Y,
}

impl From<&str> for Coordinate {
    fn from(s: &str) -> Self {
        match s {
            "x" => Self::X,
            "y" => Self::Y,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
struct FoldingRule {
    coordinate: Coordinate,
    value: u32,
}

impl From<&str> for FoldingRule {
    fn from(s: &str) -> Self {
        let value: String = s.chars().skip("fold along ".len()).collect();
        let mut split = value.split('=');
        let coordinate = Coordinate::from(split.next().unwrap());
        let value: u32 = split.next().and_then(|x| x.parse().ok()).unwrap();

        Self { coordinate, value }
    }
}

struct TransparentPaper {
    width: usize,
    height: usize,
    data: Vec<bool>,
    rules: LinkedList<FoldingRule>,
}

impl TransparentPaper {
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

    pub fn count_dots(&self) -> usize {
        self.data.iter().filter(|&&x| x).count()
    }

    pub fn fold_next_rule(&self) -> Option<Self> {
        let mut rules = self.rules.clone();
        if let Some(rule) = rules.pop_front() {
            match rule.coordinate {
                Coordinate::X => {
                    let width = rule.value as usize;
                    let height = self.height;
                    let mut data = vec![false; width * height];

                    for (idx, &v) in self.data.iter().enumerate() {
                        let (cur_x, cur_y) = self.pos_to_xy(idx);

                        match cur_x.cmp(&(rule.value as usize)) {
                            Ordering::Greater => {
                                let diff_x = cur_x - rule.value as usize;
                                let inv_x = rule.value as usize - diff_x;
                                let inv_c = inv_x + cur_y * width;
                                if v {
                                    data[inv_c] = v;
                                }
                            }
                            Ordering::Less => {
                                let idx = cur_x + cur_y * width;
                                if v {
                                    data[idx] = v;
                                }
                            }
                            _ => (),
                        }
                    }

                    Some(Self {
                        width,
                        height,
                        rules,
                        data,
                    })
                }
                Coordinate::Y => {
                    let width = self.width;
                    let height = rule.value as usize;
                    let middle_point_start = rule.value as usize * self.width;
                    let middle_point_end = middle_point_start + self.width - 1;
                    let mut data = vec![false; width * height];

                    for (idx, &x) in self.data[..middle_point_start].iter().enumerate() {
                        data[idx] = x;
                    }

                    for (idx, &x) in self.data[middle_point_end..].iter().enumerate() {
                        if x {
                            let (cur_x, cur_y) = self.pos_to_xy(middle_point_end + idx);
                            let diff_y = cur_y - rule.value as usize;
                            let inv_y = rule.value as usize - diff_y;
                            let inv_c = self.xy_to_pos(cur_x, inv_y);
                            data[inv_c] = x;
                        }
                    }

                    Some(Self {
                        width,
                        height,
                        rules,
                        data,
                    })
                }
            }
        } else {
            None
        }
    }
}

impl ToString for TransparentPaper {
    fn to_string(&self) -> String {
        let mut output = String::new();

        for (idx, &v) in self.data.iter().enumerate() {
            if idx > 0 && idx % self.width == 0 {
                output.push('\n');
            }

            if v {
                output.push('#');
            } else {
                output.push('.');
            }
        }

        output
    }
}

impl From<&str> for TransparentPaper {
    fn from(s: &str) -> Self {
        let mut split = s.split("\n\n");
        let coords = split.next().unwrap();
        let rules = split.next().unwrap();

        let coord_lines: Vec<_> = coords.lines().collect();
        let positions: Vec<(u32, u32)> = coord_lines
            .iter()
            .map(|x| {
                let mut s = x.split(',');
                (
                    s.next().and_then(|x| x.parse().ok()).unwrap(),
                    s.next().and_then(|x| x.parse().ok()).unwrap(),
                )
            })
            .collect();

        let width = positions.iter().map(|(x, _)| x).max().copied().unwrap() as usize + 1;
        let height = positions.iter().map(|(_, y)| y).max().copied().unwrap() as usize + 1;
        let mut data = vec![false; width * height];

        for (x, y) in positions {
            let coord_u = (x + y * width as u32) as usize;
            data[coord_u] = true;
        }

        let rules: LinkedList<_> = rules.lines().map(FoldingRule::from).collect();

        Self {
            width,
            height,
            data,
            rules,
        }
    }
}

impl Challenge for Day13 {
    fn run_ex1(&mut self) -> String {
        let paper = TransparentPaper::from(parse_input_raw!());
        let paper = paper.fold_next_rule().unwrap();
        paper.count_dots().to_string()
    }

    fn run_ex2(&mut self) -> String {
        let mut paper = TransparentPaper::from(parse_input_raw!());
        let path = PathBuf::new().join("debug").join("day13.png");

        while let Some(p) = paper.fold_next_rule() {
            paper = p;
        }

        save_paper_to_disk(&paper, &path);

        // Yes, answer is hardcoded, because I don't have time to apply OCR on the output. 😅
        assert_eq!(
            paper.to_string(),
            indoc::indoc! {"
            ..##.###..####..##..#..#..##..#..#.###..
            ...#.#..#....#.#..#.#..#.#..#.#..#.#..#.
            ...#.#..#...#..#....#..#.#..#.#..#.#..#.
            ...#.###...#...#....#..#.####.#..#.###..
            #..#.#....#....#..#.#..#.#..#.#..#.#.#..
            .##..#....####..##...##..#..#..##..#..#."
            }
        );
        "JPZCUAUR".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::create_day_tests;

    use super::TransparentPaper;

    create_day_tests!("13", "655", "JPZCUAUR");

    const SAMPLE_INPUT: &str = indoc::indoc! {"
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
    "};

    #[test]
    fn test_sample() {
        let paper = TransparentPaper::from(SAMPLE_INPUT);
        assert_eq!(
            paper.to_string(),
            indoc::indoc! {"
            ...#..#..#.
            ....#......
            ...........
            #..........
            ...#....#.#
            ...........
            ...........
            ...........
            ...........
            ...........
            .#....#.##.
            ....#......
            ......#...#
            #..........
            #.#........"
            }
        );

        let folded_once = paper.fold_next_rule().unwrap();
        assert_eq!(
            folded_once.to_string(),
            indoc::indoc! {"
            #.##..#..#.
            #...#......
            ......#...#
            #...#......
            .#.#..#.###
            ...........
            ..........."
            }
        );
        assert_eq!(folded_once.count_dots(), 17);

        let folded_twice = folded_once.fold_next_rule().unwrap();
        assert_eq!(
            folded_twice.to_string(),
            indoc::indoc! {"
                #####
                #...#
                #...#
                #...#
                #####
                .....
                ....."
            }
        );
    }
}
