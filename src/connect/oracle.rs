use std::vec::Vec;
use std::sync::{Arc, Mutex};
use oracle::{Connection, Error};

use crate::connect::LiteConnectConfig;

// Oracle异步连接池
pub struct OraclePool {
    pool: Arc<Mutex<Vec<Connection>>>,
    connect_config: LiteConnectConfig,
}
impl OraclePool {
    pub fn new(config: LiteConnectConfig) -> OraclePool {
        assert!(
            config.m_type == "oracle",
            "连接类型与连接池类型不匹配"
        );
        let conn = Connection::connect("scott", "tiger", "//localhost/XE")?;
        OraclePool{
            connect_config: config
        }
    }

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
    
}