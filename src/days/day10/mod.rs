//! # Day 10: Syntax Scoring
//!
//! You ask the submarine to determine the best route out of the deep-sea cave, but it only replies:
//!
//! Syntax error in navigation subsystem on line: all of them
//! All of them?! The damage is worse than you thought. You bring up a copy of the navigation subsystem (your puzzle input).
//!
//! The navigation subsystem syntax is made of several lines containing chunks. There are one or more chunks on each line, and chunks contain zero or more other chunks. Adjacent chunks are not separated by any delimiter; if one chunk stops, the next chunk (if any) can immediately start. Every chunk must open and close with one of four legal pairs of matching characters:
//!
//! If a chunk opens with (, it must close with ).
//! If a chunk opens with [, it must close with ].
//! If a chunk opens with {, it must close with }.
//! If a chunk opens with <, it must close with >.
//! So, () is a legal chunk that contains no other chunks, as is []. More complex but valid chunks include ([]), {()()()}, <([{}])>, [<>({}){}[([])<>]], and even (((((((((()))))))))).
//!
//! Some lines are incomplete, but others are corrupted. Find and discard the corrupted lines first.
//!
//! A corrupted line is one where a chunk closes with the wrong character - that is, where the characters it opens and closes with do not form one of the four legal pairs listed above.
//!
//! Examples of corrupted chunks include (], {()()()>, (((()))}, and <([]){()}[{}]). Such a chunk can appear anywhere within a line, and its presence causes the whole line to be considered corrupted.
//!
//! For example, consider the following navigation subsystem:
//!
//! [({(<(())[]>[[{[]{<()<>>
//! [(()[<>])]({[<{<<[]>>(
//! {([(<{}[<>[]}>{[]{[(<()>
//! (((({<>}<{<{<>}{[]{[]{}
//! [[<[([]))<([[{}[[()]]]
//! [{[{({}]{}}([{[{{{}}([]
//! {<[[]]>}<{[{[{[]{()[[[]
//! [<(<(<(<{}))><([]([]()
//! <{([([[(<>()){}]>(<<{{
//! <{([{{}}[<[[[<>{}]]]>[]]
//!
//! Some of the lines aren't corrupted, just incomplete; you can ignore these lines for now. The remaining five lines are corrupted:
//!
//! {([(<{}[<>[]}>{[]{[(<()> - Expected ], but found } instead.
//! [[<[([]))<([[{}[[()]]] - Expected ], but found ) instead.
//! [{[{({}]{}}([{[{{{}}([] - Expected ), but found ] instead.
//! [<(<(<(<{}))><([]([]() - Expected >, but found ) instead.
//! <{([([[(<>()){}]>(<<{{ - Expected ], but found > instead.
//!
//! Stop at the first incorrect closing character on each corrupted line.
//!
//! Did you know that syntax checkers actually have contests to see who can get the high score for syntax errors in a file? It's true! To calculate the syntax error score for a line, take the first illegal character on the line and look it up in the following table:
//!
//! ): 3 points.
//! ]: 57 points.
//! }: 1197 points.
//! >: 25137 points.
//!
//! In the above example, an illegal ) was found twice (2*3 = 6 points), an illegal ] was found once (57 points), an illegal } was found once (1197 points), and an illegal > was found once (25137 points). So, the total syntax error score for this file is 6+57+1197+25137 = 26397 points!
//!
//! Find the first illegal character in each corrupted line of the navigation subsystem. What is the total syntax error score for those errors?
//!
//! Your puzzle answer was 268845.
//!
//! ## Part Two
//!
//! Now, discard the corrupted lines. The remaining lines are incomplete.
//!
//! Incomplete lines don't have any incorrect characters - instead, they're missing some closing characters at the end of the line. To repair the navigation subsystem, you just need to figure out the sequence of closing characters that complete all open chunks in the line.
//!
//! You can only use closing characters (), ], }, or >), and you must add them in the correct order so that only legal pairs are formed and all chunks end up closed.
//!
//! In the example above, there are five incomplete lines:
//!
//! [({(<(())[]>[[{[]{<()<>> - Complete by adding }}]])})].
//! [(()[<>])]({[<{<<[]>>( - Complete by adding )}>]}).
//! (((({<>}<{<{<>}{[]{[]{} - Complete by adding }}>}>)))).
//! {<[[]]>}<{[{[{[]{()[[[] - Complete by adding ]]}}]}]}>.
//! <{([{{}}[<[[[<>{}]]]>[]] - Complete by adding ])}>.
//! Did you know that autocomplete tools also have contests? It's true! The score is determined by considering the completion string character-by-character. Start with a total score of 0. Then, for each character, multiply the total score by 5 and then increase the total score by the point value given for the character in the following table:
//!
//! ): 1 point.
//! ]: 2 points.
//! }: 3 points.
//! >: 4 points.
//! So, the last completion string above - ])}> - would be scored as follows:
//!
//! Start with a total score of 0.
//! Multiply the total score by 5 to get 0, then add the value of ] (2) to get a new total score of 2.
//! Multiply the total score by 5 to get 10, then add the value of ) (1) to get a new total score of 11.
//! Multiply the total score by 5 to get 55, then add the value of } (3) to get a new total score of 58.
//! Multiply the total score by 5 to get 290, then add the value of > (4) to get a new total score of 294.
//! The five lines' completion strings have total scores as follows:
//!
//! }}]])})] - 288957 total points.
//! )}>]}) - 5566 total points.
//! }}>}>)))) - 1480781 total points.
//! ]]}}]}]}> - 995444 total points.
//! ])}> - 294 total points.
//! Autocomplete tools are an odd bunch: the winner is found by sorting all of the scores and then taking the middle score. (There will always be an odd number of scores to consider.) In this example, the middle score is 288957 because there are the same number of scores smaller and larger than it.
//!
//! Find the completion string for each incomplete line, score the completion strings, and sort the scores. What is the middle score?
//!
//! Your puzzle answer was 4038824534.

use std::collections::{HashMap, LinkedList};

use itertools::Itertools;
use maplit::hashmap;
use once_cell::sync::Lazy;
use tap::Pipe;

use crate::{day::Challenge, parse_input_str};

/// Day 10 implementation.
pub struct Day10;

impl Challenge for Day10 {
    fn new() -> Self {
        Self
    }

    fn run_ex1(&mut self) -> String {
        NavParser::check_errors_on_lines(&parse_input_str!())
            .pipe(|x| NavParser::count_errors_score(&x).to_string())
    }

    fn run_ex2(&mut self) -> String {
        NavParser::filter_incomplete_lines(&parse_input_str!())
            .into_iter()
            .map(NavParser::autocomplete_line)
            .map(|line| NavParser::count_autocomplete_score(&line))
            .collect::<Vec<_>>()
            .pipe(NavParser::extract_middle_score)
            .to_string()
    }
}

static TOKEN_MAP: Lazy<HashMap<char, char>> = Lazy::new(|| {
    hashmap! {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
    }
});

static INVERTED_TOKEN_MAP: Lazy<HashMap<char, char>> =
    Lazy::new(|| TOKEN_MAP.iter().map(|(&x, &y)| (y, x)).collect());

static TOKEN_ERROR_VALUE: Lazy<HashMap<char, u32>> = Lazy::new(|| {
    hashmap! {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
    }
});

static TOKEN_AUTOCOMPLETE_VALUE: Lazy<HashMap<char, u64>> = Lazy::new(|| {
    hashmap! {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
    }
});

struct NavParser;

impl NavParser {
    fn get_opening_token(closing_token: char) -> char {
        INVERTED_TOKEN_MAP[&closing_token]
    }

    fn get_closing_token(opening_token: char) -> char {
        TOKEN_MAP[&opening_token]
    }

    fn autocomplete_line(line: &str) -> String {
        let mut stack: LinkedList<char> = LinkedList::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => {
                    stack.push_back(c);
                }
                ')' | ']' | '}' | '>' => {
                    stack.pop_back();
                }
                _ => unreachable!(),
            }
        }

        // Remaining characters
        let mut output = String::with_capacity(stack.len());
        while let Some(c) = stack.pop_back() {
            output.push(Self::get_closing_token(c));
        }

        output
    }

    fn check_errors_on_line(line: &str) -> Option<char> {
        let mut stack: LinkedList<char> = LinkedList::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => {
                    stack.push_back(c);
                }
                ')' | ']' | '}' | '>' => match stack.pop_back() {
                    Some(c2) => {
                        if c2 != Self::get_opening_token(c) {
                            return Some(c);
                        }
                    }
                    None => return Some(c),
                },
                _ => unreachable!(),
            }
        }

        None
    }

    fn check_errors_on_lines(lines: &[&str]) -> Vec<char> {
        lines
            .iter()
            .flat_map(|&x| Self::check_errors_on_line(x))
            .collect()
    }

    fn filter_incomplete_lines<'a>(lines: &[&'a str]) -> Vec<&'a str> {
        lines
            .iter()
            .filter(|&&x| Self::check_errors_on_line(x).is_none())
            .copied()
            .collect()
    }

    fn extract_middle_score(scores: Vec<u64>) -> u64 {
        let len = scores.len();
        let middle = len / 2;
        scores.iter().sorted().copied().nth(middle).unwrap()
    }

    fn count_autocomplete_score(completed: &str) -> u64 {
        completed
            .chars()
            .fold(0, |acc, x| acc * 5 + TOKEN_AUTOCOMPLETE_VALUE[&x])
    }

    fn count_errors_score(errors: &[char]) -> u32 {
        errors.iter().map(|x| TOKEN_ERROR_VALUE[x]).sum()
    }
}

#[cfg(test)]
mod tests {
    use tap::Pipe;

    use crate::{create_day_tests, days::day10::NavParser};

    const SAMPLE_DATA: &[&str] = &[
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];

    create_day_tests!("10", "268845", "4038824534");

    #[test]
    fn test_sample_lines() {
        assert_eq!(
            NavParser::check_errors_on_line("{([(<{}[<>[]}>{[]{[(<()>"),
            Some('}')
        );
        assert_eq!(
            NavParser::check_errors_on_line("[[<[([]))<([[{}[[()]]]"),
            Some(')')
        );
        assert_eq!(
            NavParser::check_errors_on_line("[{[{({}]{}}([{[{{{}}([]"),
            Some(']')
        );
        assert_eq!(
            NavParser::check_errors_on_line("[<(<(<(<{}))><([]([]()"),
            Some(')')
        );
        assert_eq!(
            NavParser::check_errors_on_line("<{([([[(<>()){}]>(<<{{"),
            Some('>')
        );
    }

    #[test]
    fn test_sample_score() {
        assert_eq!(
            NavParser::check_errors_on_lines(SAMPLE_DATA)
                .pipe(|x| NavParser::count_errors_score(&x)),
            26397
        );
    }

    #[test]
    fn test_sample_autocomplete() {
        assert_eq!(
            NavParser::autocomplete_line("[({(<(())[]>[[{[]{<()<>>"),
            "}}]])})]"
        );
        assert_eq!(
            NavParser::autocomplete_line("[(()[<>])]({[<{<<[]>>("),
            ")}>]})"
        );
        assert_eq!(
            NavParser::autocomplete_line("(((({<>}<{<{<>}{[]{[]{}"),
            "}}>}>))))"
        );
        assert_eq!(
            NavParser::autocomplete_line("{<[[]]>}<{[{[{[]{()[[[]"),
            "]]}}]}]}>"
        );
        assert_eq!(
            NavParser::autocomplete_line("<{([{{}}[<[[[<>{}]]]>[]]"),
            "])}>"
        );
    }

    #[test]
    fn test_sample_autocomplete_score() {
        assert_eq!(
            NavParser::filter_incomplete_lines(SAMPLE_DATA)
                .into_iter()
                .map(NavParser::autocomplete_line)
                .map(|line| NavParser::count_autocomplete_score(&line))
                .collect::<Vec<_>>()
                .pipe(NavParser::extract_middle_score),
            288957
        );
    }
}
