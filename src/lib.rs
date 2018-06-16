mod atomic;
mod mutex;
mod rwlock;

#[derive(Debug)]
pub enum Kind {
    Atomic,
    Mutex,
    RwLock,
}

impl Kind {
    pub fn try_from(s: &str) -> Result<Kind, &'static str> {
        match s.to_ascii_lowercase().as_ref() {
            "atomic" => Ok(Kind::Atomic),
            "mutex" => Ok(Kind::Mutex),
            "rwlock" => Ok(Kind::RwLock),
            _ => Err("must be one of Atomic, Mutex, RwLock"),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    kind: Kind,
    n_threads: usize,
    n_incs: usize,
}

impl Config {
    pub fn new(kind: Kind, n_threads: usize, n_incs: usize) -> Config {
        Config {
            kind,
            n_threads,
            n_incs,
        }
    }
}

pub fn run(config: Config) {
    let count = config.n_incs / config.n_threads;
    match config.kind {
        Kind::Atomic => atomic::run(config.n_threads, count),
        Kind::Mutex => mutex::run(config.n_threads, count),
        Kind::RwLock => rwlock::run(config.n_threads, count),
    }
}
