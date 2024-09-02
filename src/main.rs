use signal_hook::{
    consts::{SIGINT, SIGTERM},
    iterator::Signals,
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

mod config;
mod monitor;
mod rules;
mod userinput;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Set up signal handling
    let mut signals = Signals::new(&[SIGINT, SIGTERM]).expect("Could not create signals");
    let signal_handle = signals.handle();
    let signal_thread = std::thread::spawn({
        let r = r.clone();
        move || {
            for sig in signals.forever() {
                match sig {
                    SIGINT | SIGTERM => {
                        println!("Received signal {}, stopping...", sig);
                        r.store(false, Ordering::SeqCst);
                        break;
                    }
                    _ => unreachable!(),
                }
            }
        }
    });

    // Handle user input, which will call start_monitor or stop_monitor based on user input
    userinput::handle_input(r.clone());

    // After user input, ensure the monitoring thread stops if it was started
    r.store(false, Ordering::SeqCst);

    // Clean up
    signal_thread
        .join()
        .expect("Signal handling thread panicked");

    signal_handle.close();
    println!("Application has exited cleanly.");
}
