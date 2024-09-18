use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new `ThreadPool`.
    ///
    /// The `size` is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    #[must_use]
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let thread_cound: usize =
            thread::available_parallelism().unwrap().into();
        println!("Number of threads on this server is {thread_cound}");

        let size = if size > thread_cound {
            thread_cound
        } else {
            size
        };

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers
                .push_within_capacity(Worker::new(id, Arc::clone(&receiver)))
                .unwrap();
        }

        Self {
            workers,
            sender: Some(sender),
        }
    }

    /// # Panics
    ///
    /// panics when unable to send a job on the channel to the receiver
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // explicitly drop the sender before waiting for the threads to finish
        // Dropping sender closes the channel, which indicates no
        // more messages will be sent indicating to the workers to stop working
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("done with worker {0}, dropping worker {0}", worker.id);

            if let Some(worker) = worker.thread.take() {
                worker.join().unwrap();
            }
        }
    }
}

#[derive(Debug)]
pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        // `while let Ok(job) = receiver.lock().unwrap().recv()` doesn't
        // work because `while let` (and `if let` and `match`) does not drop
        // temporary values until the end of the associated block. So in
        // `while let`, the lock remains held for the duration of the call to
        // job() fn, meaning other workers cannot receive jobs but
        // `let job = receiver.lock().unwrap().recv().unwrap()` works because
        // with `let`, any temporary values used in the expression on
        // the right hand side of the equals sign are immediately dropped
        // when the let statement ends.
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            if let Ok(job) = message {
                println!("Worker {id} got a job; executing");
                job();
            } else {
                println!("Worker {id} is shutting down. Disconnected;");
                break;
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}
