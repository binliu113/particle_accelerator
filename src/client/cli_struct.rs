use structopt;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "pac", version = "0.0.1")]
pub struct Cli {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt)]
pub enum Command {
    /// 启动实例
    Start {},
    /// 停止实例
    Stop {},
    /// 实例列表
    List {},
    /// 创建示例
    Create {},
    /// 删除实例
    Remove {},
}
