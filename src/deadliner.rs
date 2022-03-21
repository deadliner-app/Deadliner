use crate::{
    draw_line, BACKGROUND, BLACK, GREY_WHITE, INPUT_BACKGROUND, MARGIN, PADDING, WHITE, YELLOW,
};
use chrono::{Date, Utc};
use eframe::{
    self,
    egui::{
        self, style::Margin, text, CentralPanel, ComboBox, Context, FontData, FontDefinitions,
        Frame, RichText, TextStyle,
    },
    epaint::{Color32, FontFamily, FontId, TextureHandle},
    epi::App,
};
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    time::Instant,
};

pub struct Deadliner<'a> {
    // Preloaded textures on setup to use in the lifecycle methods.
    textures: HashMap<&'a str, TextureHandle>,

    // Background Color
    background: BackgroundOptions,
    bg_color: [f32; 3],

    // How often to run?
    update_every: UpdateEvery,

    font: Font,

    date: String,

    hours: String,

    minutes: String,

    period: Periods,
}

#[derive(Debug, PartialEq)]
enum Periods {
    AM,
    PM,
}

#[derive(Debug, PartialEq)]
enum BackgroundOptions {
    Solid,
    FromDisk,
    FromURL,
}

#[derive(PartialEq, Debug)]
enum Font {
    PoppinsBlack,
    PoppinsMedium,
    PoppinsRegular,
    PoppinsLight,
}

#[derive(Debug, PartialEq)]
enum UpdateEvery {
    Day,
    Hour,
    Minute,
}

impl<'a> App for Deadliner<'a> {
    fn setup(
        &mut self,
        ctx: &Context,
        _frame: &eframe::epi::Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.load_logo_texture(ctx);
        self.set_custom_fonts(ctx);

        // ctx.set_debug_on_hover(true);
        let mut style: egui::Style = (*ctx.style()).clone();

        style.visuals.faint_bg_color = INPUT_BACKGROUND;
        style.visuals.extreme_bg_color = INPUT_BACKGROUND;
        style.visuals.override_text_color = Some(GREY_WHITE);
        ctx.set_style(style);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &eframe::epi::Frame) {
        let logo = self
            .textures
            .get("logo")
            .expect("Logo texture wasn't preloaded");

        let central_panel = CentralPanel::frame(
            CentralPanel::default(),
            Frame {
                fill: BACKGROUND,
                margin: Margin {
                    left: MARGIN,
                    right: MARGIN,
                    top: MARGIN,
                    bottom: MARGIN,
                },
                ..Default::default()
            },
        );

        central_panel.show(ctx, |ui| {
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

            draw_line(ui, 2.);

            ui.heading("Styling");

            ui.add_space(PADDING);

            ui.horizontal(|ui| {
                ui.label("Background:");

                ComboBox::from_id_source("background_options")
                    .selected_text(format!("{:?}", self.background))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.background,
                            BackgroundOptions::Solid,
                            "Solid Color",
                        );
                        ui.selectable_value(
                            &mut self.background,
                            BackgroundOptions::FromDisk,
                            "From Disk",
                        );
                        ui.selectable_value(
                            &mut self.background,
                            BackgroundOptions::FromURL,
                            "From URL",
                        );
                    });
            });

            ui.add_space(PADDING);

            match self.background {
                BackgroundOptions::Solid => {
                    ui.horizontal(|ui| {
                        ui.label("Pick a Color:");
                        ui.color_edit_button_rgb(&mut self.bg_color);
                    });
                }
                BackgroundOptions::FromURL => {
                    ui.horizontal(|ui| {
                        ui.label("Image URL:");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.date)
                                .desired_width(180.)
                                .hint_text(
                                    RichText::new("https://source.unsplash.com/random")
                                        .color(Color32::from_white_alpha(45)),
                                ),
                        );
                    });
                }
                BackgroundOptions::FromDisk => {
                    ui.horizontal(|ui| {
                        ui.label("Image Location:");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.date)
                                .desired_width(180.)
                                .hint_text(
                                    RichText::new("C:\\Users\\yassi\\Pictures\\background.png")
                                        .color(Color32::from_white_alpha(45)),
                                ),
                        );
                    });
                }
            }

            ui.add_space(PADDING);

            ui.horizontal(|ui| {
                ui.label("Update every:");
                ComboBox::from_id_source("update_every")
                    .selected_text(format!("{:?}", self.update_every))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.update_every, UpdateEvery::Day, "Day");
                        ui.selectable_value(&mut self.update_every, UpdateEvery::Hour, "Hour");
                        ui.selectable_value(&mut self.update_every, UpdateEvery::Minute, "Minute");
                    });
            });

            ui.add_space(PADDING);

            ui.horizontal(|ui| {
                ui.label("Font:");

                ComboBox::from_id_source("font_family")
                    .selected_text(format!("{:?}", self.font))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.font, Font::PoppinsBlack, "Poppins Black");
                        ui.selectable_value(&mut self.font, Font::PoppinsMedium, "Poppins Medium");
                        ui.selectable_value(
                            &mut self.font,
                            Font::PoppinsRegular,
                            "Poppins Regular",
                        );
                        ui.selectable_value(&mut self.font, Font::PoppinsLight, "Poppins Light");
                    });
            });

            ui.add_space(PADDING);

            ui.heading("Pick your Deadline");

            ui.add_space(PADDING);

            ui.horizontal_wrapped(|ui| {
                ui.label("Date:");

                ui.add(
                    egui::TextEdit::singleline(&mut self.date)
                        .desired_width(95.)
                        .hint_text(
                            RichText::new("2022-08-26").color(Color32::from_white_alpha(45)),
                        ),
                );
            });

            ui.add_space(PADDING);

            ui.horizontal_wrapped(|ui| {
                ui.label("Time:");

                ui.add(
                    egui::TextEdit::singleline(&mut self.hours)
                        .desired_width(18.)
                        .hint_text(RichText::new("7").color(Color32::from_white_alpha(45))),
                );
                ui.label(":");
                ui.add(
                    egui::TextEdit::singleline(&mut self.minutes)
                        .desired_width(18.)
                        .hint_text(RichText::new("28").color(Color32::from_white_alpha(45))),
                );

                ComboBox::from_id_source("time_period")
                    .selected_text(format!("{:?}", self.period))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.period, Periods::AM, "AM");
                        ui.selectable_value(&mut self.period, Periods::PM, "PM");
                    });
            });

            ui.add_space(20.);
            if ui
                .button(
                    RichText::new("Start!")
                        .font(FontId {
                            family: FontFamily::Name("Poppins-600".into()),
                            size: 35.,
                        })
                        .background_color(YELLOW)
                        .color(BLACK),
                )
                .clicked()
            {
                println!("Start!");
            };
        });
    }

    fn name(&self) -> &str {
        "Deadliner"
    }

    fn on_exit(&mut self) {
        println!("I'm dying..");
    }
}

impl<'a> Deadliner<'a> {
    pub fn new() -> Deadliner<'a> {
        Deadliner {
            textures: HashMap::new(),
            background: BackgroundOptions::Solid,
            bg_color: [0., 0., 0.],
            update_every: UpdateEvery::Hour,
            font: Font::PoppinsBlack,
            date: String::new(),
            hours: String::new(),
            minutes: String::new(),
            period: Periods::AM,
        }
    }

    fn set_custom_fonts(&mut self, ctx: &Context) {
        let mut fonts = FontDefinitions::default();
        let fonts_data: Vec<(&str, u16, &[u8])> = vec![
            (
                "Poppins-Regular",
                400,
                include_bytes!("../assets/fonts/Poppins-Light.ttf"),
            ),
            (
                "Poppins-Medium",
                500,
                include_bytes!("../assets/fonts/Poppins-Regular.ttf"),
            ),
            (
                "Poppins-SemiBold",
                600,
                include_bytes!("../assets/fonts/Poppins-Medium.ttf"),
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
                size: 35.,
            },
        );

        text_styles.insert(
            TextStyle::Body,
            FontId {
                family: FontFamily::Name("Poppins-400".into()),
                size: 20.,
            },
        );

        text_styles.insert(
            TextStyle::Button,
            FontId {
                family: FontFamily::Name("Poppins-400".into()),
                size: 18.,
            },
        );

        ctx.set_style(egui::Style {
            text_styles,
            ..Default::default()
        });
    }

    fn load_logo_texture(&mut self, ctx: &Context) {
        let image = image::load_from_memory(include_bytes!("../assets/icon.png")).unwrap();
        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();

        let texture = ctx.load_texture(
            "logo",
            egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()),
        );

        self.textures.insert("logo", texture);
    }
}
