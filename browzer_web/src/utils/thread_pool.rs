//! This module defines a thread pool for managing and executing tasks concurrently.
//!
//! The `thread_pool` module provides the `ThreadPool` and `Worker` structs, which are used to manage
//! a pool of worker threads that can execute tasks concurrently. The module leverages Rust's
//! standard library threading and synchronization primitives.

// external crate imports
use uuid::Uuid;

// internal crate imports
use crate::error::*;

// standard library imports
use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self},
};

/// The type of job that a worker can execute.
type Job = Box<dyn FnOnce() + Send + 'static>;

/// A struct representing a worker in the thread pool.
/// Each worker has a unique identifier and a thread.
// ----- Worker struct
#[derive(Debug)]
pub struct Worker {
    id: Uuid,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    /// This function creates a thread which runs a loop, listen for incoming jobs throught the `Receiver`, ensure
    /// the integrity of the job recieved, run the job in the thread, and return the `Worker` object
    ///
    /// # Arguments
    ///
    /// - `id` - A unique identifier for the worker.
    /// - `receiver` - A shared receiver for receiving jobs from the thread pool.
    ///
    /// # Returns
    ///
    /// A `Worker` object.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    /// use std::sync::{Arc, Mutex, mpsc};
    /// use crate::thread_pool::{Worker, Job};
    ///
    /// let (sender, receiver) = mpsc::channel();
    /// let receiver = Arc::new(Mutex::new(receiver));
    /// let worker = Worker::new(Uuid::new_v4(), Arc::clone(&receiver));
    /// ```
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

        // return the Worker struct
        return Worker {
            id,
            thread: Some(thread),
        };
    }
}

/// A struct representing a thread pool for managing worker threads.
/// The thread pool maintains a set of workers and a channel for sending jobs to them.
// ----- ThreadPool struct
#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
impl ThreadPool {
    /// This function creates a channel for sending and recieving jobs, create a vector for storing workers, and
    /// new workers accoding the `size` input provided, and return the `ThreadPool` object
    ///
    /// # Arguments
    ///
    /// - `size` - The number of workers in the thread pool. Must be greater than 0.
    ///
    /// # Returns
    ///
    /// A `ThreadPool` object.
    ///
    /// # Panics
    ///
    /// Panics if `size` is 0.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crate::thread_pool::ThreadPool;
    ///
    /// let pool = ThreadPool::new(4);
    /// ```
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker::new(Uuid::new_v4(), Arc::clone(&receiver)));
        }

        // return the ThreadPool struct
        return ThreadPool {
            sender: Some(sender),
            workers,
        };
    }

    /// Sends a job to the thread pool for execution.
    ///
    /// # Arguments
    ///
    /// - `f` - A closure representing the job to be executed.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok` if the job was successfully sent, or an `Err` if there was an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crate::thread_pool::ThreadPool;
    ///
    /// let pool = ThreadPool::new(4);
    /// pool.execute(|| {
    ///     println!("Job executed");
    /// }).unwrap();
    /// ```
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

/// The `Drop` implementation for `ThreadPool` to ensure graceful shutdown of worker threads.
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
