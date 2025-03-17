use futures::stream::{self, StreamExt};
use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;

#[test] 
async fn main(){
    // 提取数据
    let source_data = extract().await?;

}