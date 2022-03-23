use eframe::{
    egui::{RichText, Ui},
    epaint::{FontFamily, FontId, TextureHandle},
};

use crate::WHITE;

pub fn header(ui: &mut Ui, logo: &TextureHandle) {
    ui.horizontal(|ui| {
        ui.image(logo, [50., 50.]);
        ui.label(
            RichText::new("Deadliner")
                .font(FontId {
                    family: FontFamily::Name("Poppins-600".into()),
                    size: 47.,
                })
                .color(WHITE)
                .heading(),
        );
    });
}
