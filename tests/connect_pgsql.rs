use postgres::Client;
use postgres::NoTls;

#[test]
fn main(){
    let mut client = Client::connect("postgresql://postgres:Swq8855830.@localhost", NoTls).unwrap();
    let rows = client.query("SELECT 1", &[]).unwrap();
    if !rows.is_empty() {
        println!("pgsql连接成功");
    }
}