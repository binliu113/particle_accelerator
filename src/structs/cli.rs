use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "PAC")]
pub struct Cli {
    #[structopt(default_value="client",help="使用模式[serve, client]。")]
    /// 模式
    pub model: String,

    #[structopt(subcommand)]
    /// cmd
    pub cmd: Command,
}

#[derive(StructOpt)]
pub enum Command {
    /// 启动实例
    Start {},
    /// 停止实例
    Stop {},
    /// 实例列表
    List {
    },
    /// 创建示例
    Create {},
    /// 删除实例
    Remove {},
}