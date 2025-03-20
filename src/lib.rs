use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
    sender: Option<mpsc::Sender<Job>>, // Gunakan Option agar bisa di-take
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn build(size: usize) -> ThreadPool {
        assert!(size > 0, "Thread pool size must be greater than zero");

        let (sender, receiver) = mpsc::channel::<Job>(); // Tambahkan tipe Job
        let receiver = Arc::new(Mutex::new(receiver));

        let mut threads = Vec::with_capacity(size);
        for id in 0..size {
            let receiver = Arc::clone(&receiver);
            threads.push(thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv();
                match job {
                    Ok(job) => {
                        println!("Thread {id} executing job.");
                        job();
                    }
                    Err(_) => break,
                }
            }));
        }

        ThreadPool { threads, sender: Some(sender) } // Simpan sender dalam Option
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        if let Some(sender) = &self.sender {
            if sender.send(job).is_err() {
                eprintln!("Failed to send job to thread");
            }
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            drop(sender);
        }
        for thread in self.threads.drain(..) {
            thread.join().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threadpool_build() {
        let pool = ThreadPool::build(4);
        assert_eq!(pool.threads.len(), 4);
    }
}