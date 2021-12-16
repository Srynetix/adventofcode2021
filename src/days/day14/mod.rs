//! # Day 14: Extended Polymerization
//!
//! The incredible pressures at this depth are starting to put a strain on your submarine. The submarine has polymerization equipment that would produce suitable materials to reinforce the submarine, and the nearby volcanically-active caves should even have the necessary input elements in sufficient quantities.
//!
//! The submarine manual contains instructions for finding the optimal polymer formula; specifically, it offers a polymer template and a list of pair insertion rules (your puzzle input). You just need to work out what polymer would result after repeating the pair insertion process a few times.
//!
//! For example:
//!
//! NNCB
//!
//! CH -> B
//! HH -> N
//! CB -> H
//! NH -> C
//! HB -> C
//! HC -> B
//! HN -> C
//! NN -> C
//! BH -> H
//! NC -> B
//! NB -> B
//! BN -> B
//! BB -> N
//! BC -> B
//! CC -> N
//! CN -> C
//!
//! The first line is the polymer template - this is the starting point of the process.
//!
//! The following section defines the pair insertion rules. A rule like AB -> C means that when elements A and B are immediately adjacent, element C should be inserted between them. These insertions all happen simultaneously.
//!
//! So, starting with the polymer template NNCB, the first step simultaneously considers all three pairs:
//!
//! The first pair (NN) matches the rule NN -> C, so element C is inserted between the first N and the second N.
//! The second pair (NC) matches the rule NC -> B, so element B is inserted between the N and the C.
//! The third pair (CB) matches the rule CB -> H, so element H is inserted between the C and the B.
//! Note that these pairs overlap: the second element of one pair is the first element of the next pair. Also, because all pairs are considered simultaneously, inserted elements are not considered to be part of a pair until the next step.
//!
//! After the first step of this process, the polymer becomes NCNBCHB.
//!
//! Here are the results of a few steps using the above rules:
//!
//! Template:     NNCB
//! After step 1: NCNBCHB
//! After step 2: NBCCNBBBCBHCB
//! After step 3: NBBBCNCCNBBNBNBBCHBHHBCHB
//! After step 4: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB
//!
//! This polymer grows quickly. After step 5, it has length 97; After step 10, it has length 3073. After step 10, B occurs 1749 times, C occurs 298 times, H occurs 161 times, and N occurs 865 times; taking the quantity of the most common element (B, 1749) and subtracting the quantity of the least common element (H, 161) produces 1749 - 161 = 1588.
//!
//! Apply 10 steps of pair insertion to the polymer template and find the most and least common elements in the result. What do you get if you take the quantity of the most common element and subtract the quantity of the least common element?
//!
//! Your puzzle answer was 3284.
//!
//! ## Part Two
//!
//! The resulting polymer isn't nearly strong enough to reinforce the submarine. You'll need to run more steps of the pair insertion process; a total of 40 steps should do it.
//!
//! In the above example, the most common element is B (occurring 2192039569602 times) and the least common element is H (occurring 3849876073 times); subtracting these produces 2188189693529.
//!
//! Apply 40 steps of pair insertion to the polymer template and find the most and least common elements in the result. What do you get if you take the quantity of the most common element and subtract the quantity of the least common element?
//!
//! Your puzzle answer was 4302675529689.

use itertools::{Itertools, MinMaxResult};
use std::collections::{hash_map::Entry, HashMap};

use crate::{day::Challenge, parse_input_raw};

/// Day 14 implementation.
pub struct Day14;

#[derive(Debug)]
struct Polymer {
    template: String,
    pairs: HashMap<(char, char), char>,
}

struct PolymerSum<'a> {
    polymer: &'a Polymer,
    pairs_count: HashMap<(char, char), usize>,
    char_count: HashMap<char, usize>,
}

impl<'a> PolymerSum<'a> {
    pub fn new(polymer: &'a Polymer) -> Self {
        let mut pairs_count: HashMap<(char, char), usize> = HashMap::new();
        let mut char_count: HashMap<char, usize> = HashMap::new();

        let chars: Vec<_> = polymer.template.chars().collect();
        for window in chars.windows(2) {
            let pair = (window[0], window[1]);
            *pairs_count.entry(pair).or_insert(0) += 1;
        }

        for c in chars {
            *char_count.entry(c).or_insert(0) += 1;
        }

        Self {
            polymer,
            pairs_count,
            char_count,
        }
    }

    pub fn step(&mut self) {
        let mut new_pair_counts = self.pairs_count.clone();
        let mut new_char_counts = self.char_count.clone();

        for (&(p0, p1), &target) in &self.polymer.pairs {
            let mut found = false;
            let mut found_value = 0;
            if let Entry::Occupied(e) = self.pairs_count.entry((p0, p1)) {
                if *e.get() > 0 {
                    found_value = *e.get();
                    *new_pair_counts.entry((p0, p1)).or_insert(0) -= found_value;
                    found = true;
                }
            }

            if found {
                *new_pair_counts.entry((p0, target)).or_insert(0) += found_value;
                *new_pair_counts.entry((target, p1)).or_insert(0) += found_value;
                *new_char_counts.entry(target).or_insert(0) += found_value;
            }
        }

        self.char_count = new_char_counts;
        self.pairs_count = new_pair_counts;
    }

    pub fn step_for(&mut self, count: usize) {
        for _ in 0..count {
            self.step();
        }
    }

    pub fn get_common_score(&self) -> u64 {
        match self.char_count.iter().minmax_by_key(|(_, &c)| c) {
            MinMaxResult::MinMax((_, &min), (_, &max)) => max as u64 - min as u64,
            _ => unreachable!(),
        }
    }
}

impl From<&str> for Polymer {
    fn from(s: &str) -> Self {
        let mut split = s.split("\n\n");
        let template = split.next().unwrap().to_string();
        let pairs = split
            .next()
            .unwrap()
            .lines()
            .map(|x| {
                let mut split = x.split(" -> ");
                let p0 = split.next().unwrap().chars().collect::<Vec<_>>();
                let p1 = split.next().unwrap();
                ((p0[0], p0[1]), p1.chars().next().unwrap())
            })
            .collect();

        Self { template, pairs }
    }
}

impl Challenge for Day14 {
    fn run_ex1(&mut self) -> String {
        let polymer = Polymer::from(parse_input_raw!());
        let mut chain = PolymerSum::new(&polymer);
        chain.step_for(10);
        chain.get_common_score().to_string()
    }

    fn run_ex2(&mut self) -> String {
        let polymer = Polymer::from(parse_input_raw!());
        let mut chain = PolymerSum::new(&polymer);
        chain.step_for(40);
        chain.get_common_score().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::{create_day_tests, days::day14::PolymerSum};

    use super::Polymer;

    create_day_tests!("14", "3284", "4302675529689");

    const SAMPLE_DATA: &str = indoc::indoc! {"
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C"
    };

    #[test]
    fn test_sample() {
        let polymer = Polymer::from(SAMPLE_DATA);
        let mut chain = PolymerSum::new(&polymer);
        chain.step_for(10);
        assert_eq!(chain.get_common_score(), 1588);
    }

    #[test]
    fn test_sample_40() {
        let polymer = Polymer::from(SAMPLE_DATA);
        let mut chain = PolymerSum::new(&polymer);
        chain.step_for(40);
        assert_eq!(chain.get_common_score(), 2_188_189_693_529)
    }
}
