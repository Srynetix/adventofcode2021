use args::Execute;

mod args;

fn main() {
    let args: args::Args = argh::from_env();
    args.execute();
}
