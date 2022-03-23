use eframe::egui::Ui;

use crate::PADDING;

pub fn section<T>(ui: &mut Ui, header_txt: &str, add_contents: T)
where
    T: FnOnce(&mut Ui) -> (),
{
    ui.heading(header_txt);
    ui.add_space(PADDING);

    add_contents(ui);

    ui.add_space(PADDING);
}
