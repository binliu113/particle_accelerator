mod client;

use client::cli;
use client::cli_struct::Cli;
use structopt::StructOpt;

fn main() {
    let args: Cli = Cli::from_args();
    cli::cli(&args);
}
