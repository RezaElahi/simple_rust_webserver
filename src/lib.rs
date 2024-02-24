use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {

        let thread = thread::spawn(move|| {
            loop {
                let job = receiver.lock().unwrap().recv();
                match job {
                    Ok(job) => {
                        log::info!("Worker {} got a job; executing.", id);
                        job();
                    },
                    Err(_) => {
                        log::info!("Worker {} disconnected, shutting down", id);
                        break;
                    
                    }
                }
            } 
        });
        Worker { id, thread: Some(thread) }
    }
}

type Job = Box<dyn FnOnce() +  Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        // let rx = Arc::new(rx);
        
        for id in 0..size {
            let receiver = Arc::clone(&rx);

            workers.push(Worker::new(id, receiver));
        }

        ThreadPool { workers, sender: Some(tx)}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        
        std::mem::drop(self.sender.take());

        for worker in &mut self.workers {
            log::debug!("drop (shutting down) worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            } else {
                log::debug!("Worker {} already shut down", worker.id);
            }
        }
    }
}
