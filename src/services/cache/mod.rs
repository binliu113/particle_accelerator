use sqlite::Connection;

pub mod creates;

pub struct Cache {}

impl Cache {
    pub fn new(_conn: Connection) -> Cache {
        Cache {}
    }

    pub fn start() {}
}
