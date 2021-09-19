pub mod query;
pub mod insert;
pub mod update;
pub mod delete;

use sqlite::{Connection, OpenFlags};
use std::path::Path;
use std::path::PathBuf;

pub struct ConnDB {}

impl ConnDB {
    pub fn init() -> Connection {
        let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let open_flags = OpenFlags::new();
        open_flags.set_create();
        config_path.push("cache");
        config_path.push("data.db");
        let path = config_path.to_str().unwrap();
        Connection::open(path).unwrap()
    }
}