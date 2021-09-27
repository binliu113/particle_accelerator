use sqlite::Connection;

pub struct CreateClient {
    conn: Connection,
}

pub struct CreateServe {
    conn: Connection,
}

impl CreateClient {
    pub fn new(_conn: Connection) -> CreateClient {
        CreateClient {
            conn: _conn
        }
    }
    pub fn run(&self) {
        println!("{}", "cli 创建实例！！！");
    }
}

impl CreateServe {
    pub fn new(_conn: Connection) -> CreateServe {
        CreateServe {
            conn: _conn
        }
    }

    pub fn run(&self) {
        println!("{}", "ser 创建实例！！！")
    }
}