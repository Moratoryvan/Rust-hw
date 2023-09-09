use crate::waker::Signal;
use std::{
    cell::RefCell,
    collections::VecDeque,

    task::{Waker, Context}, thread::JoinHandle,
};

#[allow(unused)]
use std::future::Future;
use std::sync::Arc;
use std::task::{Poll, Wake};
use std::sync::{Mutex, mpsc};
// use async_channel;
use futures::future::BoxFuture;
// #[allow(unused)]
scoped_tls::scoped_thread_local!(static SIGNAL: Arc<Signal>);
scoped_tls::scoped_thread_local!(static RUNNABLE: Mutex<VecDeque<Arc<Task>>>);

scoped_tls::scoped_thread_local!(pub(crate) static EX: Executor);
#[allow(unused)]
pub struct Executor {
    local_queue: TaskQueue,
    thread_pool: ThreadPool,
}
#[allow(unused)]
impl Executor {
    pub fn new() -> Executor {
        Executor {
            local_queue: TaskQueue::new(),
            thread_pool: ThreadPool::new(2),
        }
    }

    pub fn spawn(fut: impl Future<Output = ()> + 'static + std::marker::Send) {
        let t = Arc::new(Task {
            future: RefCell::new(Box::pin(fut)),
            signal: Arc::new(Signal::new()),
        });
        EX.with(|ex| ex.local_queue.push(t.clone()));
    }

    pub fn block_on<F:Future>(&self, future: F) -> F::Output {
        let mut main_fut = std::pin::pin!(future);
        let signal: Arc<Signal> = Arc::new(Signal::new());
        let waker = Waker::from(signal.clone());

        let mut cx = Context::from_waker(&waker);

        EX.set(self, || {
            // pin_utils::pin_mut!(fut);
            loop {
                // return if the outer future is ready
                if let std::task::Poll::Ready(t) = main_fut.as_mut().poll(&mut cx) {
                    break t;
                }

                // consume all tasks
                while let Some(t) = self.local_queue.pop() {
                    let _ = self.thread_pool.execute(t);
                    // let future = t.future.borrow_mut();
                    // let w = waker(t.clone());
                    // let mut context = Context::from_waker(&w);
                    // let _ = Pin::new(future).as_mut().poll(&mut context);
                }

                // no task to execute now, it may ready
                if let std::task::Poll::Ready(t) = main_fut.as_mut().poll(&mut cx) {
                    break t;
                }
                signal.wait();
            }
        })

        // if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
        //     return output;
        // }

        // let runnable: Mutex<VecDeque<Arc<Task>>> = Mutex::new(VecDeque::with_capacity(1024));
        // SIGNAL.set(&signal, || {
        //     EX.set(self, || {
        //         loop {
        //             if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
        //                 return output;
        //             }
        //             while let Some(task) = runnable.lock().unwrap().pop_front() {
        //                 let waker = Waker::from(task.clone());
        //                 let mut cx = Context::from_waker(&waker);
        //                 let _ = task.future.borrow_mut().as_mut().poll(&mut cx);
        //             }
        //             signal.wait();
        //         }
        //     })
        // })
    }
}


pub struct Task {
    future: RefCell<BoxFuture<'static, ()>>,
    signal: Arc<Signal>,
}

unsafe impl Send for Task {} 
unsafe impl Sync for Task {}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        RUNNABLE.with(|runnable| runnable.lock().unwrap().push_back(self.clone()));
        self.signal.notify();
    }
}

pub struct TaskQueue {
    queue: RefCell<VecDeque<Arc<Task>>>,
}

// impl Default for TaskQueue {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl TaskQueue {
    pub fn new() -> Self {
        const DEFAULT_TASK_QUEUE_SIZE: usize = 4096;
        Self::new_with_capacity(DEFAULT_TASK_QUEUE_SIZE)
    }
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self {
            queue: RefCell::new(VecDeque::with_capacity(capacity)),
        }
    }

    pub(crate) fn push(&self, runnable: Arc<Task>) {
        // println!("add task");
        self.queue.borrow_mut().push_back(runnable);
    }

    pub(crate) fn pop(&self) -> Option<Arc<Task>> {
        // println!("remove task");
        self.queue.borrow_mut().pop_front()
    }
}
#[allow(unused)]
struct Worker {
    wid: usize,
    wthread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(wid: usize, receiver:  Arc::<Mutex<mpsc::Receiver<Option<Arc<Task>>>>>) -> Self {
        let thread = std::thread::spawn(move || {
            loop {
                let task = receiver.lock().unwrap().recv().unwrap();
                match task {
                    Some(task) => {
                        let waker = Waker::from(task.clone());
                        let mut cx = Context::from_waker(&waker);
                        let _ = task.future.borrow_mut().as_mut().poll(&mut cx);
                    },
                    None => {
                        break;
                    },
                }
            }
        });
        Worker { wid, wthread: Some(thread) }
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
    max_worker: usize,
    sender: mpsc::Sender<Option<Arc<Task>>>
}

impl ThreadPool {
    fn new(max_worker: usize) -> Self {
        if max_worker == 0 {
            panic!("max_worker must be greater than 0.")
        }
        let (sender,receiver) = mpsc::channel::<Option<Arc<Task>>>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(max_worker);
        for i in 0..max_worker {
            workers.push(Worker::new(i, receiver.clone()));
        }
        ThreadPool { workers, max_worker, sender }
    }

    fn execute(&self, task: Arc<Task>) -> Poll<()> {
        self.sender.send(Some(task)).unwrap();
        Poll::Pending
    }

}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _i in 0..self.max_worker {
            let _ = self.sender.send(None);
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.wthread.take() {
                let _ = thread.join();
            }
        }
    }
}