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

pub fn start_monitoring(running: Arc<AtomicBool>) {
    while running.load(Ordering::SeqCst) {
        // Capture and handle packets
        thread::sleep(Duration::from_millis(100));
    }
    println!("Monitoring stopped.");
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    userinput::handle_input();
}
