use oracle::Connection;

#[test]
fn main() {
    // 创建数据库连接
    let conn = Connection::connect("system", "Swq8855830.", "//172.28.182.240:1521/ORCL").unwrap();
    // 执行查询语句
    let sql = "SELECT 1 FROM dual";
    let rows = conn.query(sql, &[]).unwrap();
    for _ in rows {
        println!("oracle连接成功");
        break;
    }
}