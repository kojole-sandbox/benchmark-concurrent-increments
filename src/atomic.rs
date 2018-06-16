use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

pub fn run(n_threads: usize, count: usize) {
    let value = Arc::new(AtomicUsize::new(0));
    let mut threads = Vec::with_capacity(n_threads);

    for _ in 0..n_threads {
        let value = value.clone();
        let t = thread::spawn(move || {
            for _ in 0..count {
                value.fetch_add(1, Ordering::Relaxed);
            }
        });
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }

    assert_eq!(value.load(Ordering::Relaxed), n_threads * count);
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
