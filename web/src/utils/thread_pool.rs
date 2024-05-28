// standard library imports
use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self},
};

// external crate imports
use uuid::Uuid;

// internal crate imports
use crate::{error::*, utils::base::*};

// ----- Worker struct
#[derive(Debug)]
pub struct Worker {
    id: Uuid,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    // create a thread which runs a loop, listen for incoming jobs throught the `Receiver`, ensure
    // the integrity of the job recieved, run the job in the thread, and return the `Worker` object
    pub fn new(id: Uuid, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .map_err(ThreadPoolError::from)
                .and_then(|rx| rx.recv().map_err(ThreadPoolError::from));
            match message {
                Ok(job) => {
                    job();
                }
                Err(_) => {
                    println!("Worker {} disconnected, shutting down...", id.to_string());
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

// ----- ThreadPool struct
#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
impl ThreadPool {
    // create a channel for sending and recieving jobs, create a vector for storing workers, and
    // new workers accoding the `size` input provided, and return the `ThreadPool` object
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker::new(Uuid::new_v4(), Arc::clone(&receiver)));
        }

        ThreadPool {
            sender: Some(sender),
            workers,
        }
    }

    // send job throught the job channel using the `Sender`
    pub fn execute<F>(&self, f: F) -> Result<(), ThreadPoolError>
    where
        F: FnOnce() + Send + 'static,
    {
        let _ = self
            .sender
            .as_ref()
            .ok_or_else(|| ThreadPoolError::SendError("Sender is not innitialized".to_string()))?
            .send(Box::new(f))
            .map_err(|e| ThreadPoolError::SendError(e.to_string()));
        Ok(())
    }
}

// drop implementation for `ThreadPool`
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shuting down worker {}", worker.id.to_string());
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
