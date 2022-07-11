use crate::client;
use client::cli_struct::{Cli, Command};

pub fn cli(args: &Cli) {
    match &args.cmd {
        Command::Start { .. } => {}
        Command::Stop { .. } => {}
        Command::List { .. } => {}
        Command::Create { .. } => {}
        Command::Remove { .. } => {}
    };
}