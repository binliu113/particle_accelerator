use sqlite::Connection;

pub struct CreateClient {}

pub struct CreateServe {}

impl CreateClient {
    pub fn run(conn: Connection) {
        println!("{}", "cli 创建实例！！！");
    }
}

impl CreateServe {
    pub fn run(conn: Connection) {
        println!("{}", "ser 创建实例！！！")
    }
}