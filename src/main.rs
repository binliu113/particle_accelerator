pub mod data_models;
pub mod services;
pub mod structs;


use structopt::StructOpt;
use structs::cli::{Cli, Command};
use data_models::{ConnDB};
use sqlite::{Connection};
use services::cache::{Cache};

fn main() {
    let args: Cli = Cli::from_args();
    let conn_db: Connection = ConnDB::init();
    let cache = Cache::new(conn_db);
    match args.cmd {
        Command::Start { .. } => {}
        Command::Stop { .. } => {}
        Command::List { .. } => {}
        Command::Create { .. } => {
            cache.insert(args.model.as_str());
        }
        Command::Remove { .. } => {}
    };
}
