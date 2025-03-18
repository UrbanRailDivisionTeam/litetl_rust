use num_cpus;
use tokio;
use std::time::Duration;

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
    pub fn submit<F, R>(&self, future: F) -> tokio::task::JoinHandle<R>
    where
        F: std::future::Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        if let Some(runtime) = &self.runtime {
            runtime.spawn(future)
        } else {
            panic!("运行时未初始化");
        }
    }
}
impl Drop for LiteRuntime {
    fn drop(&mut self) {
        // 强制关停所有线程并回收任务，因为理论上程序不应当运行到这，这里能执行只有手动触发退出
        if let Some(runtime) = &self.runtime {
            let temp_runtime = std::mem::take(&mut self.runtime).unwrap();
            temp_runtime.shutdown_timeout(Duration::from_secs(60));
        }
    }
}

