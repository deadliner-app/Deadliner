#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::thread;

use deadliner_schedular::{bg_system_tray, register_auto_launch, run_server, start_schedular};

#[tokio::main]
async fn main() {
    start_schedular();

    thread::spawn(|| {
        run_server();
    });

    register_auto_launch();
    bg_system_tray();
}
