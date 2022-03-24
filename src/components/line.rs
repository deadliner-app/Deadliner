use eframe::{
    egui::{Sense, Ui},
    epaint::{pos2, Rect, Vec2},
};

use crate::SECONDARY_DARK;

pub fn render_draw_line(ui: &mut Ui, height: f32) {
    let available_space = ui.available_size_before_wrap();
    let size = Vec2::new(available_space.x, 25.);

    let (rect, response) = ui.allocate_at_least(size, Sense::hover());

    if ui.is_rect_visible(response.rect) {
        let points = [
            pos2(rect.left(), rect.center().y),
            pos2(rect.right(), rect.center().y + height),
        ];

        ui.painter().rect_filled(
            Rect {
                min: points[0],
                max: points[1],
            },
            0.0,
            SECONDARY_DARK,
        );
    }
}
