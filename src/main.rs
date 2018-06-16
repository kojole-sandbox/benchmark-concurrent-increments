extern crate coninc;

use std::env;
use std::process::exit;
use std::time::Instant;

use coninc::{run, Config, Kind};

const USAGE: &'static str = "
Benchmark of concurrent increments.

Multiple threads increments a integer concurrently in some thread-safe ways.

Usage:
  coninc [option] <kind> <n_threads> <n_incs>
  coninc (-h | --help)

Arguments:
  kind       Thread-safe implementation:
               Atomic        Use AtomicUsize.
               Mutex         Use Mutex.
               RwLock        Use RwLock.
               Channel       Send increments by mpsc channel.
               ChannelLock   Use channel-based Lock.
               CChannel      crossbeam-channel version of Channel.
               CChannelLock  crossbeam-channel version of ChannelLock.
  n_threads  # of threads.
  n_incs     # of total increments across threads.

Options:
  -h --help      Show this message.
";

fn main() {
    let config = parse_args().unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1);
    });

    let start = Instant::now();
    run(config);
    let elapsed = start.elapsed();

    println!(
        "{:.6}",
        elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1e9
    );
}

fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
        println!("{}", USAGE);
        exit(0);
    }

    if args.len() < 4 {
        println!("{}", USAGE);
        exit(1);
    }

    let kind = Kind::try_from(&args[1])
        .map_err(|err| format!("Invalid argument <kind>: {}: {}", err, args[1]))?;

    let n_threads = args[2]
        .parse::<usize>()
        .map_err(|err| format!("Invalid argument <n_threads>: {}: {}", err, args[2]))?;

    if n_threads == 0 {
        return Err(format!(
            "Invalid argument <n_threads>: must be greater than zero: {}",
            args[2]
        ));
    }

    let n_incs = args[3]
        .parse::<usize>()
        .map_err(|err| format!("Invalid argument <n_incs>: {}: {}", err, args[3]))?;

    Ok(Config::new(kind, n_threads, n_incs))
}
