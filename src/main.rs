use anyhow::Result;

mod runtime; 
use runtime::LiteRuntime;
mod connect; 
use connect::{LiteConnect, LiteConnectConfig};

mod tasks;
use tasks::Task;

fn main() -> Result<()> {
    let _temp = LiteRuntime::new();
    println!("成功运行");
    Ok(())
}
