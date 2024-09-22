use std::error::Error;
use std::fmt::{Display, Formatter, write};
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::Mutex;

enum Message {
    NewJob(Job),
    Terminate,
}

#[derive(Debug)]
pub enum PoolCreationError {
    InvalidNumberOfThreads
}

impl Display for PoolCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PoolCreationError::InvalidNumberOfThreads => {
                write!(f, "Cannot create a ThreadPool with no threads")
            }
        }
    }
}

impl Error for PoolCreationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

impl ThreadPool {
    /// Creates a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::InvalidNumberOfThreads);
        }
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate signal to all workers...");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap()
        }
        println!("Shutting down all workers:");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;


struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job! ; Executing...", id);
                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate", id);
                        break;
                    }
                }

            }
        });
        Worker { id, thread: Some(thread) }
    }
}
