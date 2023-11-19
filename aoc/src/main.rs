pub mod commands;
pub mod utils;

use commands::*;
use enum_dispatch::enum_dispatch;

use clap::Parser;

#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[enum_dispatch(CommandImpl)]
#[derive(Parser, Debug)]
enum SubCommand {
    Day1(day1::Day1),
}
fn main() -> Result<(), DynError> {
    let opts = Opts::parse();

    opts.subcommand.main()
}