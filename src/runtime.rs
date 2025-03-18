use num_cpus;
use tokio;

// 运行时部分
pub struct LiteRuntime {
    runtime: Option<tokio::runtime::Runtime>,
}
impl LiteRuntime {
    pub fn new() -> LiteRuntime {
        let cpu_cores = num_cpus::get();
        let m_runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(cpu_cores)
            .max_blocking_threads(400)
            .enable_all()
            .build()
            .unwrap();
        LiteRuntime { runtime: Some(m_runtime) }
    }
}

