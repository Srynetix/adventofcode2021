use argh::FromArgs;

pub(crate) trait Execute {
    fn execute(self);
}

/// Advent of Code runner
#[derive(FromArgs)]
pub(crate) struct Args {
    #[argh(subcommand)]
    cmd: Subcommand,
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
        println!("Running code for day {}.", self.day);
    }
}

/// run all days.
#[derive(FromArgs)]
#[argh(subcommand, name = "run-all")]
struct RunAllCommand {}

impl Execute for RunAllCommand {
    fn execute(self) {
        println!("Running code for all days.");
    }
}
