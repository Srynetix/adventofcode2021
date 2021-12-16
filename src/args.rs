//! Argument handling.

use argh::FromArgs;

use crate::{day::Challenge, run_day};

/// Execute.
pub trait Execute {
    /// Execute.
    fn execute(self);
}

/// Advent of Code runner.
#[derive(FromArgs)]
pub struct Args {
    #[argh(subcommand)]
    cmd: Subcommand,
}

impl Args {
    /// Create new "run-all" command arguments.
    pub fn new_run_all() -> Self {
        Self {
            cmd: Subcommand::RunAll(RunAllCommand {}),
        }
    }
}

impl Execute for Args {
    fn execute(self) {
        self.cmd.execute();
    }
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    RunDay(RunDayCommand),
    RunAll(RunAllCommand),
}

impl Execute for Subcommand {
    fn execute(self) {
        match self {
            Self::RunDay(x) => x.execute(),
            Self::RunAll(x) => x.execute(),
        }
    }
}

/// run day.
#[derive(FromArgs)]
#[argh(subcommand, name = "run-day")]
struct RunDayCommand {
    /// day.
    #[argh(positional)]
    day: u8,
}

impl Execute for RunDayCommand {
    fn execute(self) {
        if self.day == 0 || self.day > 25 {
            panic!("You need to choose a day between 1 and 25");
        }

        run_day_number(self.day)
    }
}

/// run all days.
#[derive(FromArgs)]
#[argh(subcommand, name = "run-all")]
struct RunAllCommand {}

impl Execute for RunAllCommand {
    fn execute(self) {
        for n in 1..=16 {
            run_day_number(n);
        }
    }
}

fn run_day_number(num: u8) {
    match num {
        1 => run_day!("01"),
        2 => run_day!("02"),
        3 => run_day!("03"),
        4 => run_day!("04"),
        5 => run_day!("05"),
        6 => run_day!("06"),
        7 => run_day!("07"),
        8 => run_day!("08"),
        9 => run_day!("09"),
        10 => run_day!("10"),
        11 => run_day!("11"),
        12 => run_day!("12"),
        13 => run_day!("13"),
        14 => run_day!("14"),
        15 => run_day!("15"),
        16 => run_day!("16"),
        _ => panic!("Day {} is unimplemented.", num),
    }
}
