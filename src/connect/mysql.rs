use mysql::*;

use crate::connect::LiteConnectConfig;

// MySQL 异步连接池
pub struct MySQLPool {
    pool: Pool,
    connect_config: LiteConnectConfig,
}
impl MySQLPool {
    // 创建一个连接池
    pub fn new(connect_config: LiteConnectConfig) -> MySQLPool {
        assert!(
            connect_config.m_type == "mysql",
            "连接类型与连接池类型不匹配"
        );
        let url = MySQLPool::make_url(&connect_config);
        let pool = Pool::new(url.as_str()).unwrap(); // 获取连接池
        MySQLPool {
            pool: pool,
            connect_config: connect_config,
        }
    }

    // 生成连接url
    fn make_url(connect_config: &LiteConnectConfig) -> String {
        let mut url = String::from("mysql://");
        url.push_str(connect_config.m_user);
        url.push_str(":");
        url.push_str(connect_config.m_password);
        url.push_str("@");
        url.push_str(connect_config.m_ip);
        url.push_str(":");
        url.push_str(connect_config.m_port);
        if !connect_config.m_database.is_empty() {
            url.push_str("/");
            url.push_str(connect_config.m_database);
        }
        url
    }

    // 获取一个连接, 连接由对应的线程自行负责回收，对于意料之外的情况直接报错，不异常处理，
    pub async fn get_conn(&self) -> PooledConn {
        self.pool.get_conn().unwrap()
    }
}
