use std::sync::mpsc::Receiver;
use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {

        let thread = thread::spawn(move|| {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job; executing.", id);
                job();  
            }
        });
        Worker { id, thread }
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
    pub fn new(size: u32) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size as usize);
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        // let rx = Arc::new(rx);
        
        for id in 0..size {
            let receiver = Arc::clone(&rx);

            workers.push(Worker::new(id as usize, receiver));
        }

        ThreadPool { workers, sender: tx}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}
