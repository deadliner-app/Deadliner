use eframe::{
    self,
    egui::{self, RichText, TextBuffer, Ui},
    epaint::Color32,
};

pub fn render_input_with_label<'a, T>(ui: &mut Ui, label: &str, value: &'a mut T, placeholder: &str)
where
    T: TextBuffer,
{
    ui.horizontal(|ui| {
        ui.label(label);

        ui.add(
            egui::TextEdit::singleline(value)
                .desired_width(95.)
                .hint_text(RichText::new(placeholder).color(Color32::from_white_alpha(20))),
        );
    });
}

pub fn render_input<'a, T>(ui: &mut Ui, value: &'a mut T, placeholder: &str, width: f32)
where
    T: TextBuffer,
{
    ui.add(
        egui::TextEdit::singleline(value)
            .desired_width(width)
            .hint_text(RichText::new(placeholder).color(Color32::from_white_alpha(20))),
    );
}
