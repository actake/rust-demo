use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

enum Message {
    Job(Job),
    Terminate,
}

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

/**
 * 一个简单线程池的实现：
 * 1. 根据指定的线程上限创建线程池并初始化线程进入等待状态
 * 2. 通过 execute 方法唤醒线程并将任务交给线程
 * 3. 线程执行任务
 * 4. 在线程池销毁时，清理所有线程下的任务
 */
impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        ThreadPool {
            workers: (0..size)
                .map(|id| Worker::new(id, Arc::clone(&receiver)))
                .collect(),
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Message::Job(Box::new(f))).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.workers.iter_mut().for_each(|_| {
            self.sender.send(Message::Terminate).unwrap();
        });

        self.workers.iter_mut().for_each(|worker| {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        });
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::Job(job) => {
                    println!("Worker {} got job, executing...", id);

                    job();
                }
                Message::Terminate => {
                    println!("worker {} receive terminate", id);
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
