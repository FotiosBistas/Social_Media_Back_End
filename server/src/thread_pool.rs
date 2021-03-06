use std::thread::{JoinHandle, Thread};
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;


pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message{
    Execute(Job),
    Terminate,
}

impl ThreadPool{
    pub fn new(size: usize) -> ThreadPool{

        assert!(size>0);

        let mut workers = Vec::with_capacity(size);

        let (sender,receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size {
            workers.push(Worker::new(i,Arc::clone(&receiver)));
        }

        ThreadPool{ workers, sender}
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers{
            println!("Shutting down worker {}",worker.id);

            if let Some(handle) = worker.thread.take() {
                handle.join().unwrap();
            };
        }
    }
}

impl ThreadPool {
    pub fn execute<F>(&self, closure: F)
        where F: FnOnce() + 'static + Send
    {
        let job = Box::new(closure);
        self.sender.send(Message::Execute(job)).unwrap();
    }
}


struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id:usize,receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker{
        let thread = thread::spawn(move || loop{
            let message = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a message; executing",id);

            match message {
                Message::Execute(job) => {
                    println!("Found a job to execute");
                    job();
                },
                Message::Terminate => {
                    println!("Terminating worker: {}", id);
                    break;
                }
            }
        });
        Worker{id,thread: Some(thread)}
    }
}