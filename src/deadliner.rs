use eframe::{
    self,
    egui::{self, CentralPanel, Context, FontData, FontDefinitions, TextStyle},
    epaint::{FontFamily, FontId},
    epi::App,
};
use std::collections::BTreeMap;

pub struct Deadliner;

impl App for Deadliner {
    fn setup(
        &mut self,
        ctx: &Context,
        _frame: &eframe::epi::Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.set_custom_fonts(ctx);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &eframe::epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Wow that was really cool");
            ui.label("Hello World");
        });
    }

    fn name(&self) -> &str {
        "Deadliner"
    }
}

impl Deadliner {
    fn set_custom_fonts(&mut self, ctx: &Context) {
        let mut fonts = FontDefinitions::default();
        let fonts_data: Vec<(&str, u16, &[u8])> = vec![
            (
                "Poppins-Regular",
                400,
                include_bytes!("../assets/fonts/Poppins-Regular.ttf"),
            ),
            (
                "Poppins-Medium",
                500,
                include_bytes!("../assets/fonts/Poppins-Medium.ttf"),
            ),
            (
                "Poppins-SemiBold",
                600,
                include_bytes!("../assets/fonts/Poppins-SemiBold.ttf"),
            ),
        ];

        // Insert all of the fonts data
        for (name, font_weight, buffer) in fonts_data {
            fonts
                .font_data
                .insert(name.to_owned(), FontData::from_static(buffer));

            fonts.families.insert(
                FontFamily::Name(format!("Poppins-{}", font_weight).into()),
                vec![name.into()],
            );
        }

        ctx.set_fonts(fonts);

        // Set text styles
        let mut text_styles = BTreeMap::new();

        text_styles.insert(
            TextStyle::Heading,
            FontId {
                family: FontFamily::Name("Poppins-500".into()),
                size: 40.,
            },
        );

        text_styles.insert(
            TextStyle::Body,
            FontId {
                family: FontFamily::Name("Poppins-400".into()),
                size: 22.,
            },
        );

        ctx.set_style(egui::Style {
            text_styles,
            ..Default::default()
        });
    }
}
