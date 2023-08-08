use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .expect("Failed to acquire receiver lock")
                .recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job. executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

/// Thread pool that manages workers
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

#[derive(Debug)]
pub struct PoolCreationError;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        Self::build(size).expect("Failed to create new thread pool")
    }

    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError);
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
    }

    /// Submits given code for execution
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        if let Some(sender) = self.sender.as_ref() {
            sender.send(job).expect("Failed to send job for execution");
        }
    }
}

/// Waits for running workers to finish
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::{
        sync::atomic::{AtomicBool, Ordering},
        thread,
        time::Duration,
    };

    #[test]
    fn should_build_thread_pool() {
        let result = ThreadPool::build(4);

        assert_eq!(result.unwrap().workers.len(), 4);
    }

    #[test]
    fn should_fail_to_build_thread_pool_with_size_0() {
        let result = ThreadPool::build(0);

        assert!(result.is_err());
    }

    #[test]
    fn should_create_new_thread_pool() {
        let pool = ThreadPool::new(4);

        assert_eq!(pool.workers.len(), 4);
    }

    #[test]
    #[should_panic]
    fn should_fail_to_create_new_thread_pool_with_size_0() {
        ThreadPool::new(0);
    }

    #[test]
    fn should_execute_given_code() {
        let executed = Arc::new(AtomicBool::new(false));

        let clone = Arc::clone(&executed);

        let pool = ThreadPool::new(2);

        pool.execute(move || {
            clone.store(true, Ordering::SeqCst);
        });

        thread::sleep(Duration::from_millis(10));

        assert!(executed.load(Ordering::SeqCst));
    }
}
