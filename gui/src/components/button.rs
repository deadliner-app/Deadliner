use eframe::{
    egui::{self, Button, RichText},
    epaint::{FontFamily, FontId},
};

use crate::{BLACK, YELLOW};

pub fn button(text: &str) -> Button {
    egui::Button::new(
        RichText::new(text)
            .font(FontId {
                family: FontFamily::Name("Poppins-600".into()),
                size: 35.,
            })
            .background_color(YELLOW)
            .color(BLACK),
    )
    .frame(false)
    .fill(YELLOW)
}
