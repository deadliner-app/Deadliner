#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::fs;

use deadliner_gui::{new_path, Deadliner};
use eframe::{
    epaint::{Pos2, Vec2},
    run_native, NativeOptions,
};
use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Icon, WindowBuilder},
};

fn main() {
    // Setup deadliner dir for cache beforehand
    let cache_dir = dirs::cache_dir().ok_or("no cache dir").unwrap();
    let deadliner_cache = cache_dir.join("deadliner");

    if !deadliner_cache.exists() {
        fs::create_dir(deadliner_cache).unwrap();
    }

    let app = Deadliner::new();

    let icon = image::open(new_path("assets/icon.png"))
        .expect("Failed to open icon path")
        .to_rgba8();

    let (icon_width, icon_height) = icon.dimensions();

    // Get the primary screen dimensions
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_window_icon(Some(
            Icon::from_rgba(icon.clone().into_raw(), icon_width, icon_height).unwrap(),
        ))
        .with_visible(false)
        .build(&event_loop)
        .unwrap();

    let PhysicalSize { width, height } = window.primary_monitor().unwrap().size();

    // Set the initial window position at the very bottom right
    let app_width = 400.;
    let app_height = 630.;
    let taskbar_approx_height = 65.;
    let padding = 15.;

    let win_options = NativeOptions {
        initial_window_size: Some(Vec2::new(app_width, app_height)),
        initial_window_pos: Some(Pos2 {
            x: width as f32 - app_width - padding,
            y: height as f32 - app_height - padding - taskbar_approx_height,
        }),
        ..Default::default()
    };

    run_native(Box::new(app), win_options);
}
