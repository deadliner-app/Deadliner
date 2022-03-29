#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    sync::{Arc, Mutex},
    thread,
};

use deadliner_schedular::{bg_system_tray, register_auto_launch, run_server, start_schedular};

#[tokio::main]
async fn main() {
    let exit = Arc::new(Mutex::new(false));

    let sched_exit = Arc::clone(&exit);
    start_schedular(sched_exit);

    // a Mutex exit value to trigger graceful shutdown when `/shutdown` endpoint
    // is hit by the gui.
    let server_exit = Arc::clone(&exit);
    thread::spawn(move || {
        run_server(server_exit);
    });

    register_auto_launch();
    bg_system_tray(exit);
}
