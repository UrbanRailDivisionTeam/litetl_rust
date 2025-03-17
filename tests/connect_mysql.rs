use mysql::Pool; 
use mysql::prelude::Queryable;

#[test]
fn main() {  
    let pool = Pool::new("mysql://swq:Swq8855830.@172.30.240.39:3306").unwrap();  
    let mut conn = pool.get_conn().unwrap();  
    let results: Vec<i32> = conn.query("SELECT 1").unwrap();
    if !results.is_empty() {
        println!("mysql连接成功");
    }
}  