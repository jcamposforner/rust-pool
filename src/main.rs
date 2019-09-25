use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::vec::Vec;

struct ThreadBuilder<F>
where
    F: Send + 'static + Fn(usize) -> usize + Sync,
{
    func: F,
}

impl<F> ThreadBuilder<F>
where
    F: Send + 'static + Fn(usize) -> usize + Sync,
{
    fn new(func: F) -> Self {
        ThreadBuilder { func }
    }

    fn execute(self, i: usize) -> JoinHandle<usize> {
        let func = Arc::new(self.func);

        thread::spawn(move || func(i))
    }
}

fn a(i: usize) -> usize {
    println!("thread #{}", i);
    i
}

fn main() {
    let mut handlers: Vec<JoinHandle<usize>> = Vec::new();
    for i in 0..10 {
        handlers.push(ThreadBuilder::new(a).execute(i));
    }

    for handler in handlers {
        let results = handler.join().unwrap();

        println!("{}", results);
    }
}
