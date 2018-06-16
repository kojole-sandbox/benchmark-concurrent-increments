use std::sync::{Arc, RwLock};
use std::thread;

pub fn run(n_threads: usize, count: usize) -> usize {
    let value = Arc::new(RwLock::new(0usize));
    let mut threads = Vec::with_capacity(n_threads);

    for _ in 0..n_threads {
        let value = value.clone();
        let t = thread::spawn(move || {
            for _ in 0..count {
                *value.write().unwrap() += 1;
            }
        });
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }

    let v = *value.read().unwrap();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_single() {
        assert_eq!(run(1, 1_000), 1_000);
    }

    #[test]
    fn run_concurrent() {
        assert_eq!(run(4, 250), 1_000);
    }
}
