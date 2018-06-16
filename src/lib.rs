mod atomic;

#[derive(Debug)]
pub enum Kind {
    Atomic,
}

impl Kind {
    pub fn try_from(s: &str) -> Result<Kind, &'static str> {
        match s.to_ascii_lowercase().as_ref() {
            "atomic" => Ok(Kind::Atomic),
            _ => Err("must be one of atomic"),
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
    }
}
