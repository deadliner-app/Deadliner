#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::thread;

use deadliner_schedular::{bg_system_tray, register_auto_launch, start_schedular};

#[tokio::main]
async fn main() {
    let sched = start_schedular();

    register_auto_launch();
    bg_system_tray();
}
