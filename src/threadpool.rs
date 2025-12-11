use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread::{self, JoinHandle},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let sender = Some(sender);
        let receiver = Arc::new(Mutex::new(receiver));
        let workers: Vec<_> = (0..size)
            .map(|id| Worker::new(id, receiver.clone()))
            .collect();

        Self { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .as_ref()
            .unwrap()
            .send(job)
            .expect("failed to send job");
    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Self { id, thread }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().expect("failed to shutdown worker")
        }
    }
}
