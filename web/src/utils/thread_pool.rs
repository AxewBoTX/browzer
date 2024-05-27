use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use uuid::Uuid;

use crate::utils::base::*;

// Worker struct
#[derive(Debug)]
pub struct Worker {
    id: Uuid,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    pub fn new(id: Uuid, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
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

// ThreadPool struct
#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
impl ThreadPool {
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
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let _ = self.sender.as_ref().unwrap().send(Box::new(f));
    }
}
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
