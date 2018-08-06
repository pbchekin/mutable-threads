extern crate linebased;
extern crate threadpool;

use linebased::Server;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};
use threadpool::ThreadPool;

fn main() {
    let threads = 2;
    let pool = ThreadPool::new(threads);
    let one_second = time::Duration::from_secs(1);
    let muted = Arc::new(AtomicBool::new(false));
    let shutdown = Arc::new(AtomicBool::new(false));

    for id in 0..threads {
        let muted = muted.clone();
        let shutdown = shutdown.clone();
        pool.execute(move || {
            let mut counter = 0;
            while !shutdown.load(Ordering::Relaxed) {
                if !muted.load(Ordering::Relaxed) {
                    println!("{}: {}", id, counter);
                }
                counter = counter + 1;
                thread::sleep(one_second);
            }
        })
    }

    let mut server = Server::new(Default::default(), move |query| match query {
        "version" => String::from("0.1.0"),
        "shutdown" => {
            shutdown.store(true, Ordering::Relaxed);
            String::from("requested shutdown")
        }
        "mute" => {
            muted.store(true, Ordering::Relaxed);
            String::from("muted")
        }
        "unmute" => {
            muted.store(false, Ordering::Relaxed);
            String::from("unmuted")
        }
        _ => String::from("unknown command"),
    }).unwrap();

    let handle = server.handle();
    let thread = thread::spawn(move || server.run().unwrap());

    pool.join();
    handle.shutdown().unwrap();
    thread.join().unwrap();
}
