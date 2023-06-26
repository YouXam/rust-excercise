use std::{thread, sync::{mpsc::{self, Receiver}, Mutex, Arc}};


#[allow(dead_code)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<Receiver<Job>>>) -> Worker {
        Worker {
            id,
            thread: Some(thread::spawn(move || {
                loop {
                    let job = rx.lock().unwrap().recv();
                    match job {
                        Ok(job) => {
                            println!("Worker {} got a job; executing.", id);
                            job();
                        },
                        _ => {
                            println!("Worker {} disconnected; shutting down...", id);
                            return;
                        }
                    }
                }
            })),
        }
    }
}

#[allow(dead_code)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

/// 线程池
impl ThreadPool {
    /// 创建线程池。
    ///
    /// 线程池中线程的数量。
    ///
    /// # Panics
    ///
    /// `new` 函数在 size 为 0 时会 panic。
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rx))) 
            
        }
        return ThreadPool { workers, sender: Some(tx) };
    }
    /// 尝试使用一个线程运行一个闭包函数
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
    /// 等待所有的线程结束后，终止线程池
    pub fn terminate(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.terminate();
    }
}