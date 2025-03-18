pub mod clickhouse;
pub mod mongo;
pub mod mysql;
pub mod oracle;
pub mod pgsql;
pub mod sqlserver;

// 以下是connect包本身的相关代码
use std::any::Any;

/*
    异种数据库连接的配置信息
    因为公司的服务相关配置几乎不会变更，所以相关信息直接硬编码到程序中，不设置对应的配置文件
    基于此，所有的字符串均为静态生命周期
*/
#[derive(Copy, Clone)]
pub struct LiteConnectConfig {
    m_type: &'static str,
    m_user: &'static str,
    m_password: &'static str,
    m_ip: &'static str,
    m_port: &'static str,
    m_database: &'static str,
}

// 异种数据库连接抽象
pub struct LiteConnect {
    // 连接名称，其静态原因同上
    m_name: &'static str,
    m_config: LiteConnectConfig,
    // 对所有连接进行类型擦除后的指针
    m_connect: Box<dyn Any>,
}
impl LiteConnect {
    pub fn new(name: &'static str, config: LiteConnectConfig) -> LiteConnect {
        let m_connect: Box<dyn Any>;
        match config.m_type {
            "mysql" => {
                m_connect = Box::new(mysql::MySQLPool::new(config.clone()));
            }
            "pgsql" => {
                m_connect = Box::new(());
            }
            "oracle" => {
                m_connect = Box::new(());
            }
            "sqlserver" => {
                m_connect = Box::new(());
            }
            "mongo" => {
                m_connect = Box::new(());
            }
            "clickhouse" => {
                m_connect = Box::new(());
            }
            _ => {
                panic!("不支持的数据库类型");
            }
        };
        LiteConnect {
            m_name: name,
            m_config: config,
            m_connect: m_connect,
        }
    }
}
