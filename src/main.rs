pub mod data_models;
pub mod services;
pub mod structs;

use services::creates;
use structopt::StructOpt;
use structs::cli::{Cli, Command};
use data_models::conn_db::{ConnDB};
use sqlite::{Connection};

fn main() {
    let args: Cli = Cli::from_args();
    let conn_db = ConnDB::init();
    match &args.model.as_str() {
        &"serve" => {
            match args.cmd {
                Command::Start { .. } => {}
                Command::Stop { .. } => {}
                Command::List { .. } => {}
                Command::Create { .. } => {
                    creates::CreateServe::run(conn_db);
                }
                Command::Remove { .. } => {}
            };
        }
        &"client" => {
            match args.cmd {
                Command::Start { .. } => {}
                Command::Stop { .. } => {}
                Command::List { .. } => {}
                Command::Create { .. } => {
                    creates::CreateClient::run(conn_db);
                }
                Command::Remove { .. } => {}
            };
        }
        _ => {}
    };
}
