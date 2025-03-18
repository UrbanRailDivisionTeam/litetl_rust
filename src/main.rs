use anyhow::Result;

mod utils; 
use utils::runtime::LiteRuntime;

fn main() -> Result<()> {
    let _temp = LiteRuntime::new()?;
    println!("成功运行");
    Ok(())
}
