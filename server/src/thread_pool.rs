use std::thread::{JoinHandle, Thread};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;


pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::channel(Message),
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message{
    Execute(Job),
    Fail,
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

            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
            };
        }
    }
}

impl<F> ThreadPool
    where F: FnOnce() + 'static + Send
{
    pub fn execute(&self,closure: F){
        let job = Box::new(closure);
        self.sender.send(Message::Execute(job)).unwrap();
    }
}


struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id:usize,receiver: Arc<Mutex<mpsc::receiver<Message>>>) -> Worker{
        let thread = thread::spawn(move || loop{
            let message = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing",id);

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing",id);
                    job;
                },
                Message::Terminate => {
                    break;
                }
            }
        });
        Worker{id,thread: Some(thread)}
    }
}