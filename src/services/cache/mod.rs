use sqlite::Connection;

mod creates;

pub struct Cache {
    conn: Connection,
}

impl Cache {
    pub fn new(_conn: Connection) -> Cache {
        Cache {
            conn: _conn
        }
    }

    pub fn insert(&self,mode:&str) {
        println!("{}",mode);
    }
}
