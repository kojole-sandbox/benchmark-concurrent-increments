use std::sync::{Arc, Mutex};
use std::thread;

pub fn run(n_threads: usize, count: usize) {
    let value = Arc::new(Mutex::new(0usize));
    let mut threads = Vec::with_capacity(n_threads);

    for _ in 0..n_threads {
        let value = value.clone();
        let t = thread::spawn(move || {
            for _ in 0..count {
                *value.lock().unwrap() += 1;
            }
        });
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }

    assert_eq!(*value.lock().unwrap(), n_threads * count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_single() {
        run(1, 1_000);
    }

    #[test]
    fn run_concurrent() {
        run(4, 250);
    }
}
