use eframe::{
    egui::{self, Button, RichText},
    epaint::{Color32, FontFamily, FontId},
};

pub fn button(text: &str, color: Color32, bg: Color32, font_weight: u16, font_size: f32) -> Button {
    egui::Button::new(
        RichText::new(text)
            .font(FontId {
                family: FontFamily::Name(
                    ("Poppins-".to_string() + font_weight.to_string().as_str()).into(),
                ),
                size: font_size,
            })
            .color(color),
    )
    .frame(false)
    .fill(bg)
}
