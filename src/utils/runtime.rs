use anyhow::Result;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use tokio;

// 工作线程部分
type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Worker> {
        let thread = thread::spawn(move || loop {
            let lock = receiver.lock().unwrap();
            let job = lock.recv().unwrap();
            job();
        });
        Ok(Worker { id, thread })
    }
}

// 线程池部分
pub struct ThreadPool {
    is_break: bool,
    workers: Vec<Option<Worker>>,
    sender: mpsc::Sender<Job>,
}
impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool> {
        assert!(size > 0, "线程池大小必须大于0");

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            let temp_work = Worker::new(id, Arc::clone(&receiver))?;
            workers.push(Some(temp_work));
        }
        let is_break = false;
        Ok(ThreadPool { is_break, workers, sender })
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            let temp_worker = std::mem::take(worker).unwrap();
            temp_worker.thread.join().unwrap();
        }
    }
}

// 运行时部分
pub struct LiteRuntime {
    extract_runtime: tokio::runtime::Runtime,
    load_runtime: tokio::runtime::Runtime,
    process_runtime: ThreadPool,
}
impl LiteRuntime {
    pub fn new() -> Result<LiteRuntime> {
        let extract_runtime = tokio::runtime::Builder::new_current_thread().build()?;
        let load_runtime = tokio::runtime::Builder::new_current_thread().build()?;
        let parallelism = thread::available_parallelism()?;
        let parallelism_num = parallelism.get();
        let mut computer_num = 1;
        if parallelism_num > 2 {
            computer_num = parallelism_num - 2
        }
        let process_runtime = ThreadPool::new(computer_num)?;
        Ok(LiteRuntime {
            extract_runtime,
            load_runtime,
            process_runtime,
        })
    }
}
