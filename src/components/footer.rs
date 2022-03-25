use eframe::{
    egui::{style::Margin, Context, Frame, TopBottomPanel, Ui},
    epaint::{Color32, TextureHandle},
};

use crate::{MARGIN, PADDING};

pub fn render_footer(ctx: &Context, ui: &mut Ui, github_img: &TextureHandle) {
    ui.add_space(PADDING * 2.5);

    let bottom_panel = TopBottomPanel::frame(
        TopBottomPanel::bottom("footer"),
        Frame {
            fill: Color32::from_rgb(9, 14, 23),
            margin: Margin {
                left: MARGIN,
                right: MARGIN,
                top: MARGIN / 1.5,
                bottom: MARGIN / 1.5,
            },
            ..Default::default()
        },
    );

    bottom_panel.show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.image(github_img, [18., 18.]);
            ui.label("Give me a ⭐ here");
            ui.hyperlink_to("Source Code", "https://github.com/YassinEldeeb/deadliner");
        });
    });

    ui.add_space(PADDING);
}
