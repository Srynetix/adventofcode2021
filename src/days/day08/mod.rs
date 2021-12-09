//! # Day 8: Seven Segment Search
//!
//! You barely reach the safety of the cave when the whale smashes into the cave mouth, collapsing it. Sensors indicate another exit to this cave at a much greater depth, so you have no choice but to press on.
//!
//! As your submarine slowly makes its way through the cave system, you notice that the four-digit seven-segment displays in your submarine are malfunctioning; they must have been damaged during the escape. You'll be in a lot of trouble without them, so you'd better figure out what's wrong.
//!
//! Each digit of a seven-segment display is rendered by turning on or off any of seven segments named a through g:
//!
//!   0:      1:      2:      3:      4:
//!  aaaa    ....    aaaa    aaaa    ....
//! b    c  .    c  .    c  .    c  b    c
//! b    c  .    c  .    c  .    c  b    c
//!  ....    ....    dddd    dddd    dddd
//! e    f  .    f  e    .  .    f  .    f
//! e    f  .    f  e    .  .    f  .    f
//!  gggg    ....    gggg    gggg    ....
//!
//!   5:      6:      7:      8:      9:
//!  aaaa    aaaa    aaaa    aaaa    aaaa
//! b    .  b    .  .    c  b    c  b    c
//! b    .  b    .  .    c  b    c  b    c
//!  dddd    dddd    ....    dddd    dddd
//! .    f  e    f  .    f  e    f  .    f
//! .    f  e    f  .    f  e    f  .    f
//!  gggg    gggg    ....    gggg    gggg
//!
//! So, to render a 1, only segments c and f would be turned on; the rest would be off. To render a 7, only segments a, c, and f would be turned on.
//!
//! The problem is that the signals which control the segments have been mixed up on each display. The submarine is still trying to display numbers by producing output on signal wires a through g, but those wires are connected to segments randomly. Worse, the wire/segment connections are mixed up separately for each four-digit display! (All of the digits within a display use the same connections, though.)
//!
//! So, you might know that only signal wires b and g are turned on, but that doesn't mean segments b and g are turned on: the only digit that uses two segments is 1, so it must mean segments c and f are meant to be on. With just that information, you still can't tell which wire (b/g) goes to which segment (c/f). For that, you'll need to collect more information.
//!
//! For each display, you watch the changing signals for a while, make a note of all ten unique signal patterns you see, and then write down a single four digit output value (your puzzle input). Using the signal patterns, you should be able to work out which pattern corresponds to which digit.
//!
//! For example, here is what you might see in a single entry in your notes:
//!
//! acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
//! cdfeb fcadb cdfeb cdbaf
//!
//! (The entry is wrapped here to two lines so it fits; in your notes, it will all be on a single line.)
//!
//! Each entry consists of ten unique signal patterns, a | delimiter, and finally the four digit output value. Within an entry, the same wire/segment connections are used (but you don't know what the connections actually are). The unique signal patterns correspond to the ten different ways the submarine tries to render a digit using the current wire/segment connections. Because 7 is the only digit that uses three segments, dab in the above example means that to render a 7, signal lines d, a, and b are on. Because 4 is the only digit that uses four segments, eafb means that to render a 4, signal lines e, a, f, and b are on.
//!
//! Using this information, you should be able to work out which combination of signal wires corresponds to each of the ten digits. Then, you can decode the four digit output value. Unfortunately, in the above example, all of the digits in the output value (cdfeb fcadb cdfeb cdbaf) use five segments and are more difficult to deduce.
//!
//! For now, focus on the easy digits. Consider this larger example:
//!
//! be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
//! fdgacbe cefdb cefbgd gcbe
//! edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
//! fcgedb cgb dgebacf gc
//! fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
//! cg cg fdcagb cbg
//! fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
//! efabcd cedba gadfec cb
//! aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
//! gecf egdcabf bgf bfgea
//! fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
//! gebdcfa ecba ca fadegcb
//! dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
//! cefg dcbef fcge gbcadfe
//! bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
//! ed bcgafe cdgba cbgef
//! egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
//! gbdfcae bgc cg cgb
//! gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
//! fgae cfgab fg bagce
//!
//! Because the digits 1, 4, 7, and 8 each use a unique number of segments, you should be able to tell which combinations of signals correspond to those digits. Counting only digits in the output values (the part after | on each line), in the above example, there are 26 instances of digits that use a unique number of segments (highlighted above).
//!
//! In the output values, how many times do digits 1, 4, 7, or 8 appear?
//!
//! Your puzzle answer was 412.
//!
//! ## Part Two
//!
//! Through a little deduction, you should now be able to determine the remaining digits. Consider again the first example above:
//!
//! acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
//! cdfeb fcadb cdfeb cdbaf
//!
//! After some careful analysis, the mapping between signal wires and segments only make sense in the following configuration:
//!
//!  dddd
//! e    a
//! e    a
//!  ffff
//! g    b
//! g    b
//!  cccc
//!
//! So, the unique signal patterns would correspond to the following digits:
//!
//! acedgfb: 8
//! cdfbe: 5
//! gcdfa: 2
//! fbcad: 3
//! dab: 7
//! cefabd: 9
//! cdfgeb: 6
//! eafb: 4
//! cagedb: 0
//! ab: 1
//!
//! Then, the four digits of the output value can be decoded:
//!
//! cdfeb: 5
//! fcadb: 3
//! cdfeb: 5
//! cdbaf: 3
//!
//! Therefore, the output value for this entry is 5353.
//!
//! Following this same process for each entry in the second, larger example above, the output value of each entry can be determined:
//!
//! fdgacbe cefdb cefbgd gcbe: 8394
//! fcgedb cgb dgebacf gc: 9781
//! cg cg fdcagb cbg: 1197
//! efabcd cedba gadfec cb: 9361
//! gecf egdcabf bgf bfgea: 4873
//! gebdcfa ecba ca fadegcb: 8418
//! cefg dcbef fcge gbcadfe: 4548
//! ed bcgafe cdgba cbgef: 1625
//! gbdfcae bgc cg cgb: 8717
//! fgae cfgab fg bagce: 4315
//!
//! Adding all of the output values in this larger example produces 61229.
//!
//! For each entry, determine all of the wire/segment connections and decode the four-digit output values. What do you get if you add up all of the output values?
//!
//! Your puzzle answer was 978171.

use crate::{day::Challenge, parse_input_str};
use itertools::Itertools;

/// Day 08 implementation.
pub struct Day08;

// Array-based segment solver.
//
//  0000
// 1    2
// 1    2
//  3333
// 4    5
// 4    5
//  6666

#[derive(Debug)]
struct Pattern(String);
#[derive(Debug)]
struct PatternLine {
    signal_patterns: Vec<Pattern>,
    output_patterns: Vec<Pattern>,
}
struct PatternCounter;

#[derive(Debug, Clone)]
struct SolutionTree {
    solution: String,
    children: Vec<SolutionTree>,
}

impl SolutionTree {
    pub fn leaves_at_depth(&self, depth: usize) -> Vec<&SolutionTree> {
        Self::leaves_at_depth_rec(0, depth, self)
    }

    fn leaves_at_depth_rec(index: usize, depth: usize, tree: &Self) -> Vec<&SolutionTree> {
        if index == depth {
            tree.children.iter().collect()
        } else {
            let mut output = vec![];
            for child in &tree.children {
                let values = Self::leaves_at_depth_rec(index + 1, depth, child);
                for value in values {
                    output.push(value);
                }
            }
            output
        }
    }
}

fn is_digit_valid(solution: &str, next_digit: u8, next_pattern: &Pattern) -> bool {
    let solution_chars: Vec<char> = solution.chars().collect();
    let positions = get_digit_positions(next_digit);
    for perm in next_pattern.0.chars().permutations(next_pattern.0.len()) {
        let mut valid = true;
        let mut potential_solution = solution_chars.clone();
        for (idx, v) in perm.iter().enumerate() {
            let pos = positions[idx];
            if solution_chars[pos] != '.' && solution_chars[pos] != *v {
                valid = false;
            } else {
                potential_solution[pos] = *v;
            }
        }

        if valid {
            return true;
        }
    }

    false
}

fn get_digit_positions(digit: u8) -> &'static [usize] {
    match digit {
        0 => &[0, 1, 2, 4, 5, 6],
        1 => &[2, 5],
        2 => &[0, 2, 3, 4, 6],
        3 => &[0, 2, 3, 5, 6],
        4 => &[1, 2, 3, 5],
        5 => &[0, 1, 3, 5, 6],
        6 => &[0, 1, 3, 4, 5, 6],
        7 => &[0, 2, 5],
        8 => &[0, 1, 2, 3, 4, 5, 6],
        9 => &[0, 1, 2, 3, 5, 6],
        _ => panic!("Unsupported digit {}", digit),
    }
}

fn get_digit_from_position(position: &[usize]) -> u8 {
    for d in 0..=9 {
        if get_digit_positions(d) == position {
            return d;
        }
    }

    panic!("Position not found {:?}", position)
}

fn extract_known_patterns<'a>(
    patterns: &[&'a Pattern],
) -> (Vec<(u8, &'a Pattern)>, Vec<&'a Pattern>) {
    let mut known_patterns = vec![];
    let mut ambiguous_patterns = vec![];

    for &pattern in patterns {
        if let Some(d) = guess_digit(pattern) {
            known_patterns.push((d, pattern));
        } else {
            ambiguous_patterns.push(pattern);
        }
    }

    (known_patterns, ambiguous_patterns)
}

fn find_next_solutions(
    current_solution: &str,
    next_digit: u8,
    next_pattern: &Pattern,
) -> Vec<String> {
    let solution_chars: Vec<char> = current_solution.chars().collect();
    let positions = get_digit_positions(next_digit);
    let mut output = vec![];

    for perm in next_pattern.0.chars().permutations(next_pattern.0.len()) {
        let mut valid = true;
        let mut potential_solution = solution_chars.clone();
        for (idx, v) in perm.iter().enumerate() {
            let pos = positions[idx];
            if solution_chars[pos] != '.' && solution_chars[pos] != *v {
                valid = false;
            } else {
                potential_solution[pos] = *v;
            }
        }

        if valid {
            output.push(potential_solution.iter().collect());
        }
    }

    output
}

fn generate_solution_tree(known_patterns: &[(u8, &Pattern)]) -> SolutionTree {
    let mut init = SolutionTree {
        children: vec![],
        solution: ".......".into(),
    };

    generate_known_solution_step(&mut init, known_patterns);
    init
}

fn generate_potential_solutions(known_patterns: &[(u8, &Pattern)]) -> Vec<String> {
    let tree = generate_solution_tree(known_patterns);
    tree.leaves_at_depth(known_patterns.len() - 1)
        .iter()
        .map(|x| x.solution.clone())
        .collect()
}

fn generate_known_solution_step(tree: &mut SolutionTree, remaining_patterns: &[(u8, &Pattern)]) {
    let (next_digit, next_pattern) = remaining_patterns[0];
    let remaining_patterns = &remaining_patterns[1..];
    let solutions = find_next_solutions(&tree.solution, next_digit, next_pattern);

    for solution in solutions {
        tree.children.push(SolutionTree {
            children: vec![],
            solution,
        });
    }

    if !remaining_patterns.is_empty() {
        for child in &mut tree.children {
            generate_known_solution_step(child, remaining_patterns);
        }
    }
}

fn generate_ambiguous_possibilities<'a>(
    remaining_patterns: &[&'a Pattern],
) -> Vec<Vec<(u8, &'a Pattern)>> {
    let sorted_patterns: Vec<&Pattern> = remaining_patterns
        .iter()
        .copied()
        .sorted_by(|a, b| a.0.len().cmp(&b.0.len()))
        .collect();
    let mut possibilities = vec![];

    // First three are 2, 3, 5
    let first_digits = [2, 3, 5];
    let first_patterns = &sorted_patterns[..3];
    // Last three are 0, 6, 9
    let last_digits = [0, 6, 9];
    let last_patterns = &sorted_patterns[3..];

    let indexes: [usize; 3] = [0, 1, 2];
    let mut first_perms = vec![];
    let mut last_perms = vec![];
    for perm in indexes.iter().permutations(3) {
        let mut first_perms_vec = vec![];
        let mut last_perms_vec = vec![];
        for (idx, &&i) in perm.iter().enumerate() {
            let first_digit = first_digits[idx];
            let first_pattern = first_patterns[i];
            first_perms_vec.push((first_digit, first_pattern));

            let last_digit = last_digits[idx];
            let last_pattern = last_patterns[i];
            last_perms_vec.push((last_digit, last_pattern));
        }

        first_perms.push(first_perms_vec);
        last_perms.push(last_perms_vec);
    }

    // Zip two perm arrays
    for first_perm_vec in &first_perms {
        for last_perm_vec in &last_perms {
            let mut possibility = vec![];
            for &f in first_perm_vec {
                possibility.push(f);
            }
            for &l in last_perm_vec {
                possibility.push(l);
            }
            possibilities.push(possibility);
        }
    }

    possibilities
}

fn decode_pattern(solution: &str, pattern: &str) -> u8 {
    // Compute positions
    let solution_chars = solution.chars().collect::<Vec<char>>();
    let position = pattern
        .chars()
        .map(|x| {
            solution_chars
                .iter()
                .position(|&y| x == y)
                .expect("Position should be valid")
        })
        .sorted()
        .collect::<Vec<_>>();
    get_digit_from_position(&position)
}

fn guess_digit(pattern: &Pattern) -> Option<u8> {
    match pattern.0.len() {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}

fn bruteforce_ambiguous_patterns(
    possible_solutions: &[String],
    remaining_patterns: Vec<Vec<(u8, &Pattern)>>,
) -> String {
    for solution in possible_solutions.iter() {
        for remaining_pattern in remaining_patterns.iter() {
            let mut valid = true;
            for (digit, pattern) in remaining_pattern {
                if !is_digit_valid(solution, *digit, pattern) {
                    valid = false;
                    break;
                }
            }

            if valid {
                return solution.to_string();
            }
        }
    }

    panic!("Could not find solution.")
}

impl PatternCounter {
    pub fn count_unambiguous_output_patterns(lines: &[PatternLine]) -> usize {
        lines
            .iter()
            .map(|x| x.count_unambiguous_output_patterns())
            .sum()
    }
}

impl Pattern {
    fn guess_digit(&self) -> Option<u8> {
        match self.0.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        }
    }
}

impl PatternLine {
    pub fn count_unambiguous_output_patterns(&self) -> usize {
        self.output_patterns
            .iter()
            .flat_map(|x| x.guess_digit())
            .count()
    }

    pub fn compute_solution(&self) -> String {
        let patterns = self.signal_patterns.iter().collect::<Vec<_>>();
        let (known_patterns, ambiguous_patterns) = extract_known_patterns(&patterns);
        let potential_solutions = generate_potential_solutions(&known_patterns);
        let possibilities = generate_ambiguous_possibilities(&ambiguous_patterns);
        bruteforce_ambiguous_patterns(&potential_solutions, possibilities)
    }

    pub fn decode_output(&self, solution: &str) -> String {
        self.output_patterns
            .iter()
            .map(|x| decode_pattern(solution, &x.0))
            .join("")
    }
}

impl From<&str> for Pattern {
    fn from(s: &str) -> Self {
        Self(s.chars().sorted().collect())
    }
}

impl From<&str> for PatternLine {
    fn from(s: &str) -> Self {
        let mut spl = s.split(" | ");
        let signals: Vec<Pattern> = spl
            .next()
            .expect("Could not extract signals from input")
            .split_whitespace()
            .map(Into::into)
            .collect();
        let output: Vec<Pattern> = spl
            .next()
            .expect("Could not extract output from input")
            .split_whitespace()
            .map(Into::into)
            .collect();

        Self {
            signal_patterns: signals,
            output_patterns: output,
        }
    }
}

impl Challenge for Day08 {
    fn new() -> Self {
        Self
    }

    fn run_ex1(&mut self) -> String {
        let lines: Vec<PatternLine> = parse_input_str!().iter().map(|&x| x.into()).collect();
        PatternCounter::count_unambiguous_output_patterns(&lines).to_string()
    }

    fn run_ex2(&mut self) -> String {
        let lines: Vec<PatternLine> = parse_input_str!().iter().map(|&x| x.into()).collect();
        lines
            .iter()
            .map(|x| {
                let solution = x.compute_solution();
                x.decode_output(&solution)
                    .parse::<usize>()
                    .expect("Should be a valid number")
            })
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::create_day_tests;

    use super::{PatternCounter, PatternLine};

    create_day_tests!("08", "412", "978171");

    const SAMPLE_DATA: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    const LARGER_SAMPLE_DATA: &[&str] = &[
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    ];

    #[test]
    fn test_sample() {
        let lines = PatternLine::from(SAMPLE_DATA);
        assert_eq!(
            PatternCounter::count_unambiguous_output_patterns(&[lines]),
            0
        );
    }

    #[test]
    fn test_larger_sample() {
        let lines: Vec<PatternLine> = LARGER_SAMPLE_DATA.iter().map(|&x| x.into()).collect();
        assert_eq!(
            PatternCounter::count_unambiguous_output_patterns(&lines),
            26
        );
    }

    #[test]
    fn test_sample_solving() {
        let line = PatternLine::from(SAMPLE_DATA);
        assert_eq!(line.compute_solution(), "deafgbc");
    }

    #[test]
    fn test_sample_decoding() {
        let line = PatternLine::from(SAMPLE_DATA);
        let solution = "deafgbc";
        assert_eq!(line.decode_output(solution), "5353");
    }

    #[test]
    fn test_larger_sample_decoding() {
        let outputs = [
            "8394", "9781", "1197", "9361", "4873", "8418", "4548", "1625", "8717", "4315",
        ];

        let lines: Vec<PatternLine> = LARGER_SAMPLE_DATA.iter().map(|&x| x.into()).collect();
        let computed: Vec<_> = lines
            .iter()
            .map(|x| {
                let solution = x.compute_solution();
                x.decode_output(&solution)
            })
            .collect();

        assert_eq!(computed, outputs);
    }
}
