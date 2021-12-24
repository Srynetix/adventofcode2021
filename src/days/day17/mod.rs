//! # Day 17: Trick Shot
//!
//! You finally decode the Elves' message. HI, the message says. You continue searching for the sleigh keys.
//!
//! Ahead of you is what appears to be a large ocean trench. Could the keys have fallen into it? You'd better send a probe to investigate.
//!
//! The probe launcher on your submarine can fire the probe with any integer velocity in the x (forward) and y (upward, or downward if negative) directions. For example, an initial x,y velocity like 0,10 would fire the probe straight up, while an initial velocity like 10,-1 would fire the probe forward at a slight downward angle.
//!
//! The probe's x,y position starts at 0,0. Then, it will follow some trajectory by moving in steps. On each step, these changes occur in the following order:
//!
//! The probe's x position increases by its x velocity.
//! The probe's y position increases by its y velocity.
//! Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it is already 0.
//! Due to gravity, the probe's y velocity decreases by 1.
//! For the probe to successfully make it into the trench, the probe must be on some trajectory that causes it to be within a target area after any step. The submarine computer has already calculated this target area (your puzzle input). For example:
//!
//! target area: x=20..30, y=-10..-5
//! This target area means that you need to find initial x,y velocity values such that after any step, the probe's x position is at least 20 and at most 30, and the probe's y position is at least -10 and at most -5.
//!
//! Given this target area, one initial velocity that causes the probe to be within the target area after any step is 7,2:
//!
//! .............#....#............
//! .......#..............#........
//! ...............................
//! S........................#.....
//! ...............................
//! ...............................
//! ...........................#...
//! ...............................
//! ....................TTTTTTTTTTT
//! ....................TTTTTTTTTTT
//! ....................TTTTTTTT#TT
//! ....................TTTTTTTTTTT
//! ....................TTTTTTTTTTT
//! ....................TTTTTTTTTTT
//!
//! In this diagram, S is the probe's initial position, 0,0. The x coordinate increases to the right, and the y coordinate increases upward. In the bottom right, positions that are within the target area are shown as T. After each step (until the target area is reached), the position of the probe is marked with #. (The bottom-right # is both a position the probe reaches and a position in the target area.)
//!
//! Another initial velocity that causes the probe to be within the target area after any step is 6,3:
//!
//! ...............#..#............
//! ...........#........#..........
//! ...............................
//! ......#..............#.........
//! ...............................
//! ...............................
//! S....................#.........
//! ...............................
//! ...............................
//! ...............................
//! .....................#.........
//! ....................TTTTTTTTTTT
//! ....................TTTTTTTTTTT
//! ....................TTTTTTTTTTT
//! ....................TTTTTTTTTTT
//! ....................T#TTTTTTTTT
//! ....................TTTTTTTTTTT
//!
//! Another one is 9,0:
//!
//! S........#.....................
//! .................#.............
//! ...............................
//! ........................#......
//! ...............................
//! ....................TTTTTTTTTTT
//! ....................TTTTTTTTTT#
//! ....................TTTTTTTTTTT
//! ....................TTTTTTTTTTT
//! ....................TTTTTTTTTTT
//! ....................TTTTTTTTTTT
//!
//! One initial velocity that doesn't cause the probe to be within the target area after any step is 17,-4:
//!
//! S..............................................................
//! ...............................................................
//! ...............................................................
//! ...............................................................
//! .................#.............................................
//! ....................TTTTTTTTTTT................................
//! ....................TTTTTTTTTTT................................
//! ....................TTTTTTTTTTT................................
//! ....................TTTTTTTTTTT................................
//! ....................TTTTTTTTTTT..#.............................
//! ....................TTTTTTTTTTT................................
//! ...............................................................
//! ...............................................................
//! ...............................................................
//! ...............................................................
//! ................................................#..............
//! ...............................................................
//! ...............................................................
//! ...............................................................
//! ...............................................................
//! ...............................................................
//! ...............................................................
//! ..............................................................#
//!
//! The probe appears to pass through the target area, but is never within it after any step. Instead, it continues down and to the right - only the first few steps are shown.
//!
//! If you're going to fire a highly scientific probe out of a super cool probe launcher, you might as well do it with style. How high can you make the probe go while still reaching the target area?
//!
//! In the above example, using an initial velocity of 6,9 is the best you can do, causing the probe to reach a maximum y position of 45. (Any higher initial y velocity causes the probe to overshoot the target area entirely.)
//!
//! Find the initial velocity that causes the probe to reach the highest y position and still eventually be within the target area after any step. What is the highest y position it reaches on this trajectory?

use once_cell::sync::Lazy;
use regex::Regex;

use crate::{day::Challenge, parse_input_raw};

/// Day 17 implementation.
pub struct Day17;

impl Challenge for Day17 {
    fn run_ex1(&mut self) -> String {
        let rect = Rect::from_input(parse_input_raw!());
        Launcher::determine_highest_y(&rect).to_string()
    }

    fn run_ex2(&mut self) -> String {
        "".to_string()
    }
}

static INPUT_RGX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"target area: x=(?P<x1>-?\d+)..(?P<x2>-?\d+), y=(?P<y1>-?\d+)..(?P<y2>-?\d+)")
        .unwrap()
});

#[derive(Debug, PartialEq)]
struct Rect {
    x: i64,
    y: i64,
    width: i64,
    height: i64,
}

impl Rect {
    pub fn new(x: i64, y: i64, width: i64, height: i64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn from_input(input: &str) -> Self {
        let capt = INPUT_RGX.captures(input).unwrap();
        let x1: i64 = capt["x1"].parse().unwrap();
        let x2: i64 = capt["x2"].parse().unwrap();
        let y1: i64 = capt["y1"].parse().unwrap();
        let y2: i64 = capt["y2"].parse().unwrap();

        let min_y = y1.min(y2);
        let max_y = y1.max(y2);

        Self {
            x: x1,
            y: max_y,
            width: (x2 - x1).abs(),
            height: (max_y - min_y).abs(),
        }
    }

    pub fn left(&self) -> i64 {
        self.x
    }

    pub fn top(&self) -> i64 {
        self.y
    }

    pub fn right(&self) -> i64 {
        self.x + self.width
    }

    pub fn bottom(&self) -> i64 {
        self.y + self.height
    }
}

#[derive(Debug, PartialEq)]
enum OvershootAxis {
    X,
    Y,
}

#[derive(Debug, PartialEq)]
struct CollisionResult {
    collided: bool,
    overshoot: Option<OvershootAxis>,
    steps: usize,
    y_max: i64,
}

struct Launcher;

impl Launcher {
    pub fn launch_and_collide(
        target_area: &Rect,
        velocity_x: i64,
        velocity_y: i64,
    ) -> CollisionResult {
        let mut probe = Probe::new(velocity_x, velocity_y);
        let mut result = CollisionResult {
            overshoot: None,
            collided: false,
            steps: 0,
            y_max: 0,
        };

        loop {
            let overshoot = Self::has_overshoot(target_area, probe.position_x, probe.position_y);
            if let Some(o) = overshoot {
                result.overshoot = Some(o);
                result.collided = false;
                break;
            }

            let contact = Self::has_contact(target_area, probe.position_x, probe.position_y);
            if contact {
                result.collided = true;
                break;
            }

            probe.step();
            result.y_max = result.y_max.max(probe.position_y);
            result.steps += 1;
        }

        result
    }

    fn has_overshoot(
        target_area: &Rect,
        position_x: i64,
        position_y: i64,
    ) -> Option<OvershootAxis> {
        let x_overshoot = if target_area.left() < 0 {
            position_x < target_area.left()
        } else {
            position_x > target_area.right()
        };

        let y_overshoot = if target_area.top() < 0 {
            position_y < target_area.bottom()
        } else {
            position_y > target_area.top()
        };

        if x_overshoot {
            Some(OvershootAxis::X)
        } else if y_overshoot {
            Some(OvershootAxis::Y)
        } else {
            None
        }
    }

    fn has_contact(target_area: &Rect, position_x: i64, position_y: i64) -> bool {
        let target_right_x = target_area.x + target_area.width;
        let target_bottom_y = target_area.y + target_area.height;

        position_x >= target_area.x
            && position_x <= target_right_x
            && position_y >= target_area.y
            && position_y <= target_bottom_y
    }

    pub fn determine_highest_y(target_area: &Rect) -> i64 {
        let mut max_y = i64::MIN;
        let offset = if target_area.left() < 0 { -1 } else { 1 };
        let mut cur_x = offset;
        let mut cur_y = 1;

        println!("{:?}", target_area);

        loop {
            // println!("Trying ({}, {})", cur_x, cur_y);

            let res = Self::launch_and_collide(target_area, cur_x, cur_y);
            if res.collided {
                if res.y_max > max_y {
                    max_y = res.y_max;
                }
                cur_y += 1;
                // println!("Collision at ({}, {}) with {} steps", cur_x, cur_y, res.steps);
            } else if let Some(o) = res.overshoot {
                match o {
                    OvershootAxis::X => {
                        // Too strong
                        cur_x -= 1;
                    }
                    OvershootAxis::Y => {
                        // Too high or not strong enough on x
                        cur_y -= 1;
                        cur_x += 1;
                    }
                }
            }

            if target_area.left() < 0 && cur_x < target_area.left() {
                return max_y;
            }

            if target_area.right() > 0 && cur_x > target_area.right() {
                return max_y;
            }
        }
    }
}

struct Probe {
    velocity_x: i64,
    velocity_y: i64,
    position_x: i64,
    position_y: i64,
}

impl Probe {
    pub fn new(velocity_x: i64, velocity_y: i64) -> Self {
        Self {
            velocity_x,
            velocity_y,
            position_x: 0,
            position_y: 0,
        }
    }

    pub fn step(&mut self) {
        self.position_x += self.velocity_x;
        self.position_y += self.velocity_y;

        if self.velocity_x > 0 {
            self.velocity_x -= 1;
        } else if self.velocity_x < 0 {
            self.velocity_x += 1;
        }

        self.velocity_y -= 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::{create_day_tests, days::day17::CollisionResult};

    use super::{Launcher, Rect};

    create_day_tests!("17", "", "");

    const SAMPLE_DATA: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_sample() {
        let rect = Rect::from_input(SAMPLE_DATA);
        assert_eq!(
            Launcher::launch_and_collide(&rect, 7, 2),
            CollisionResult {
                overshoot: None,
                collided: true,
                steps: 7,
                y_max: 3
            }
        );
        assert_eq!(
            Launcher::launch_and_collide(&rect, 6, 3),
            CollisionResult {
                overshoot: None,
                collided: true,
                steps: 9,
                y_max: 6
            }
        );
        assert_eq!(
            Launcher::launch_and_collide(&rect, 9, 0),
            CollisionResult {
                overshoot: None,
                collided: true,
                steps: 4,
                y_max: 0
            }
        );
        assert_eq!(
            Launcher::launch_and_collide(&rect, 17, -4),
            CollisionResult {
                overshoot: None,
                collided: false,
                steps: 2,
                y_max: 0
            }
        );
        assert_eq!(
            Launcher::launch_and_collide(&rect, 6, 9),
            CollisionResult {
                overshoot: None,
                collided: true,
                steps: 20,
                y_max: 45
            }
        );
    }

    #[test]
    fn test_determine_highest_y() {
        let rect = Rect::from_input(SAMPLE_DATA);
        assert_eq!(Launcher::determine_highest_y(&rect), 45);
    }
}
