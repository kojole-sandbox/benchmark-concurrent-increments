use std::collections::VecDeque;
use std::sync::mpsc::{channel, Sender};
use std::thread;

use ptr::Sendable;

enum Lock {
    Lock,
    Unlock,
}

type ResponseSender = Sender<Sendable<usize>>;

struct Request {
    lock: Lock,
    res_tx: Option<ResponseSender>,
}

impl Request {
    fn lock(res_tx: ResponseSender) -> Request {
        Request {
            lock: Lock::Lock,
            res_tx: Some(res_tx),
        }
    }

    fn unlock() -> Request {
        Request {
            lock: Lock::Unlock,
            res_tx: None,
        }
    }
}

pub fn run(n_threads: usize, count: usize) -> usize {
    let (req_tx, req_rx) = channel();
    let mut threads = Vec::with_capacity(n_threads);

    for _ in 0..n_threads {
        let req_tx = req_tx.clone();

        let t = thread::spawn(move || {
            let (res_tx, res_rx) = channel();

            for _ in 0..count {
                req_tx.send(Request::lock(res_tx.clone())).unwrap();
                let mut ptr = res_rx.recv().unwrap();
                *ptr.as_mut() += 1;
                req_tx.send(Request::unlock()).unwrap();
            }
        });

        threads.push(t);
    }
    drop(req_tx);

    let mut value = 0usize;
    let mut queue: VecDeque<ResponseSender> = VecDeque::new();
    for req in req_rx {
        match req.lock {
            Lock::Lock => {
                // Serve only if there is no serving thread
                if queue.is_empty() {
                    req.res_tx
                        .as_ref()
                        .unwrap()
                        .send(Sendable::new(&mut value))
                        .unwrap();
                }
                queue.push_back(req.res_tx.unwrap());
            }
            Lock::Unlock => {
                // Delete serving thread in queue
                queue.pop_front().unwrap();

                // Serve another thread in queue if exists
                if let Some(req_tx) = queue.front() {
                    req_tx.send(Sendable::new(&mut value)).unwrap();
                }
            }
        }
    }

    for t in threads {
        t.join().unwrap();
    }

    value
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
