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

        match self.day {
            1 => run_day!("01"),
            2 => run_day!("02"),
            3 => run_day!("03"),
            4 => run_day!("04"),
            5 => run_day!("05"),
            6 => run_day!("06"),
            7 => run_day!("07"),
            8 => run_day!("08"),
            _ => panic!("Day {} is unimplemented.", self.day),
        }
    }
}

/// run all days.
#[derive(FromArgs)]
#[argh(subcommand, name = "run-all")]
struct RunAllCommand {}

impl Execute for RunAllCommand {
    fn execute(self) {
        run_day!("01");
        run_day!("02");
        run_day!("03");
        run_day!("04");
        run_day!("05");
        run_day!("06");
        run_day!("07");
        run_day!("08");
    }
}
