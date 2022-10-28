use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread::{spawn, JoinHandle},
};

pub struct ThreadPool {
    exec_queue: Arc<Mutex<VecDeque<Box<dyn FnOnce() -> () + Send>>>>,
    handles: Vec<JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(n_threads: usize) -> Self {
        let exec_queue: Arc<Mutex<VecDeque<Box<dyn FnOnce() -> () + Send>>>> =
            Arc::new(Mutex::new(VecDeque::new()));
        let mut handles = vec![];

        for _ in 0..n_threads {
            let exec_queue = Arc::clone(&exec_queue);
            handles.push(spawn(move || loop {
                let mut exec_queue_guard = exec_queue.lock().unwrap();
                if let Some(request) = exec_queue_guard.pop_front() {
                    drop(exec_queue_guard);
                    request();
                };
            }));
        }

        Self {
            exec_queue,
            handles,
        }
    }

    pub fn execute<R: FnOnce() -> () + Send + 'static>(&mut self, request: R) {
        self.exec_queue.lock().unwrap().push_back(Box::new(request));
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        while let Some(handle) = self.handles.pop() {
            handle.join().unwrap();
        }
    }
}
