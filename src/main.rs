use std::{
    fs::File,
    io::{BufReader, Read},
};

use deadline::update_wallpaper;
use eframe::{
    self,
    egui::{self, CentralPanel, Style, Visuals},
    epaint::{Pos2, Vec2},
    epi::{App, IconData},
    run_native, NativeOptions,
};

struct Deadline;

impl App for Deadline {
    fn update(&mut self, ctx: &egui::Context, frame: &eframe::epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Wow that was really cool");
            ui.label("Hello World");
        });
    }

    fn name(&self) -> &str {
        "Deadlines"
    }
}

fn main() {
    let app = Deadline;

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

    // update_wallpaper("3 Days, 14 Hours Left.");
}
