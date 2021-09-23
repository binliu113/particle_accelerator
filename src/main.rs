pub mod data_models;
pub mod services;
pub mod structs;


use structopt::StructOpt;
use structs::cli::{Cli, Command};
use data_models::{ConnDB};
use sqlite::{Connection};

fn main() {
    let args: Cli = Cli::from_args();
    let conn_db: Connection = ConnDB::init();
    // let _servers = Services
    services::Cache::new(conn_db);
    match &args.model.as_str() {
        &"serve" => {
            match args.cmd {
                Command::Start { .. } => {}
                Command::Stop { .. } => {}
                Command::List { .. } => {}
                Command::Create { .. } => {}
                Command::Remove { .. } => {}
            };
        }
        &"client" => {
            match args.cmd {
                Command::Start { .. } => {}
                Command::Stop { .. } => {}
                Command::List { .. } => {}
                Command::Create { .. } => {}
                Command::Remove { .. } => {}
            };
        }
        _ => {}
    };
}
