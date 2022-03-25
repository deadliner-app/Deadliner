#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::fs;

use deadliner::Deadliner;
use eframe::{
    epaint::{Pos2, Vec2},
    epi::IconData,
    run_native, NativeOptions,
};

fn main() {
    // Setup deadliner dir for cache beforehand
    let cache_dir = dirs::cache_dir().ok_or("no cache dir").unwrap();
    let deadliner_cache = cache_dir.join("deadliner");

    if !deadliner_cache.exists() {
        fs::create_dir(deadliner_cache).unwrap();
    }

    let app = Deadliner::new();

    let icon = image::open("assets/icon.png")
        .expect("Failed to open icon path")
        .to_rgba8();

    let (icon_width, icon_height) = icon.dimensions();

    let win_options = NativeOptions {
        initial_window_pos: Some(Pos2 { x: 1500., y: 370. }),
        initial_window_size: Some(Vec2::new(400., 630.)),
        icon_data: Some(IconData {
            width: icon_width,
            height: icon_height,
            rgba: icon.into_raw(),
        }),
        ..Default::default()
    };

    run_native(Box::new(app), win_options);
}
