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
pub(crate) struct Args {
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
        let implemented_days = 1;

        if self.day == 0 || self.day > 25 {
            panic!("You need to choose a day between 1 and 25");
        }

        if self.day > implemented_days {
            panic!("Code for day {} is unimplemented.", self.day);
        }

        match self.day {
            1 => {
                run_day!("01")
            }
            _ => unreachable!(),
        }
    }
}

/// run all days.
#[derive(FromArgs)]
#[argh(subcommand, name = "run-all")]
struct RunAllCommand {}

impl Execute for RunAllCommand {
    fn execute(self) {
        run_day!("01")
    }
}