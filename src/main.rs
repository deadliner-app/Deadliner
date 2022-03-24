#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use deadliner::{update_wallpaper, Deadliner};
use eframe::{
    epaint::{Pos2, Vec2},
    epi::IconData,
    run_native, NativeOptions,
};

fn main() {
    let app = Deadliner::new();

    let icon = image::open("assets/icon.png")
        .expect("Failed to open icon path")
        .to_rgba8();

    let (icon_width, icon_height) = icon.dimensions();

    let win_options = NativeOptions {
        initial_window_pos: Some(Pos2 { x: 1500., y: 400. }),
        initial_window_size: Some(Vec2::new(400., 600.)),
        icon_data: Some(IconData {
            width: icon_width,
            height: icon_height,
            rgba: icon.into_raw(),
        }),
        ..Default::default()
    };

    run_native(Box::new(app), win_options);
}
