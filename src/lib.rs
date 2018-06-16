extern crate crossbeam_channel;
extern crate parking_lot;

mod atomic;
mod cchannel;
mod cchannel_lock;
mod channel;
mod channel_lock;
mod mutex;
mod pl_mutex;
mod pl_rwlock;
mod ptr;
mod rwlock;

#[derive(Debug)]
pub enum Kind {
    Atomic,
    Mutex,
    RwLock,
    PlMutex,
    PlRwLock,
    Channel,
    ChannelLock,
    CChannel,
    CChannelLock,
}

impl Kind {
    pub fn try_from(s: &str) -> Result<Kind, &'static str> {
        match s.to_ascii_lowercase().as_ref() {
            "atomic" => Ok(Kind::Atomic),
            "mutex" => Ok(Kind::Mutex),
            "rwlock" => Ok(Kind::RwLock),
            "plmutex" => Ok(Kind::PlMutex),
            "plrwlock" => Ok(Kind::PlRwLock),
            "channel" => Ok(Kind::Channel),
            "channellock" => Ok(Kind::ChannelLock),
            "cchannel" => Ok(Kind::CChannel),
            "cchannellock" => Ok(Kind::CChannelLock),
            _ => Err("must be one of Atomic, Mutex, RwLock, Channel, ChannelLock, CChannel, CChannelLock"),
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
        Kind::PlMutex => pl_mutex::run(config.n_threads, count),
        Kind::PlRwLock => pl_rwlock::run(config.n_threads, count),
        Kind::Channel => channel::run(config.n_threads, count),
        Kind::ChannelLock => channel_lock::run(config.n_threads, count),
        Kind::CChannel => cchannel::run(config.n_threads, count),
        Kind::CChannelLock => cchannel_lock::run(config.n_threads, count),
    }
}
