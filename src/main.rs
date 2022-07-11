mod client;

use structopt::StructOpt;
use client::cli_struct::Cli;
use client::cli;

fn main() {
    let args: Cli = Cli::from_args();
    cli::cli(&args);
}
