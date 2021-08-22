use sqlite::{Connection};

pub struct ConnDB {}

impl ConnDB {
    pub fn init() -> Connection {
        Connection::open("./cache/data.db").unwrap()
    }
}