extern crate linebased;
extern crate threadpool;
extern crate regex;

use linebased::Server;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::{thread, time};
use threadpool::ThreadPool;
use regex::Regex;
use std::time::{Duration, Instant};

fn main() {
    let threads = 2;
    let pool = ThreadPool::new(threads);
    let one_second = time::Duration::from_secs(1);

    let muted : Arc<RwLock<Option<Instant>>> = Arc::new(RwLock::new(None));
    let shutdown = Arc::new(AtomicBool::new(false));

    for id in 0..threads {
        let muted = muted.clone();
        let shutdown = shutdown.clone();
        pool.execute(move || {
            let mut counter = 0;
            while !shutdown.load(Ordering::Relaxed) {
                let muted = match *muted.read().unwrap_or_else(|e| e.into_inner()) {
                    None => false,
                    Some(ts) => Instant::now() < ts
                };

                if !muted {
                    println!("{}: {}", id, counter);
                }

                counter = counter + 1;
                thread::sleep(one_second);
            }
        })
    }

    let mute_re = Regex::new(r"mute (\d+)").unwrap();

    let mut server = Server::new(Default::default(), move |query| {
        match query {
            "shutdown" => {
                shutdown.store(true, Ordering::Relaxed);
            }
            "unmute" => {
                *muted.write().unwrap_or_else(|e| e.into_inner()) = None
            }
            _ => {
                if let Some(mute_cap) = mute_re.captures(query) {
                    if let Ok(delay) = mute_cap[1].parse::<u64>() {
                        *muted.write().unwrap_or_else(|e| e.into_inner()) = Some(Instant::now() + Duration::from_secs(delay));
                    } else {
                        return String::from("invalid delay")
                    }
                } else {
                    return String::from("unknown command")
                }
            }
        }
        "ok".to_string()
    }).unwrap();

    let handle = server.handle();
    let thread = thread::spawn(move || server.run().unwrap());

    pool.join();
    handle.shutdown().unwrap();
    thread.join().unwrap();
}
