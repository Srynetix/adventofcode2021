//! Advent of Code 2021

#![deny(missing_docs)]

use args::Execute;

pub mod args;
pub mod day;
pub mod days;

fn main() {
    color_eyre::install().expect("Could not install color-eyre.");

    let args: args::Args = argh::from_env();
    args.execute();
}

#[cfg(test)]
mod tests {
    use crate::args::{Args, Execute};

    #[test]
    fn test_run_all() {
        let args = Args::new_run_all();
        args.execute();
    }
}
