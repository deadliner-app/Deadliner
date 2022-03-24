use crate::{
    button, get_file_name_from_path, is_string_numeric, render_draw_line, render_header,
    render_input, render_input_with_label, render_section, sanitize_inputs, BACKGROUND, GREY_WHITE,
    MARGIN, PADDING, SECONDARY, SECONDARY_BRIGHT, SECONDARY_DARK, YELLOW,
};
use chrono::NaiveDateTime;
use eframe::{
    self,
    egui::{
        self,
        style::{Margin, Selection, WidgetVisuals},
        CentralPanel, ComboBox, Context, FontData, FontDefinitions, Frame, RichText, TextStyle,
    },
    epaint::{Color32, FontFamily, FontId, Rounding, Stroke, TextureHandle},
    epi::App,
};
use image::GenericImageView;
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    fs,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct DeadlinerConf {
    pub background: BackgroundOptions,
    pub wallpaper_mode: WallpaperMode,

    pub bg_color: [u8; 3],
    pub bg_url: String,
    pub bg_location: String,

    pub update_every: UpdateEvery,

    pub font: Font,
    pub font_size: u8,
    pub font_color: [u8; 3],

    pub date: String,
    pub hours: String,
    pub minutes: String,
    pub period: Periods,
}

pub struct Deadliner<'a> {
    // Preloaded textures on setup to use in the lifecycle methods.
    textures: HashMap<&'a str, TextureHandle>,
    screen: (i32, i32),

    error_msg: String,
    invalid_bg: bool,

    conf: DeadlinerConf,
}

#[derive(Debug, PartialEq, Copy, Clone, EnumIter)]
pub enum Periods {
    AM,
    PM,
}

#[derive(Debug, PartialEq, Copy, Clone, EnumIter)]
pub enum BackgroundOptions {
    Solid,
    FromDisk,
    FromURL,
}

#[derive(PartialEq, Debug, Clone, Copy, EnumIter)]
pub enum Font {
    PoppinsBlack,
    PoppinsMedium,
    PoppinsRegular,
    PoppinsLight,
}

#[derive(Debug, PartialEq, Clone, Copy, EnumIter)]
pub enum UpdateEvery {
    Day,
    Hour,
    Minute,
}

#[derive(Debug, PartialEq, Clone, Copy, EnumIter)]
pub enum WallpaperMode {
    Center,
    Crop,
    Fit,
    Span,
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
        let mut style = (*ctx.style()).clone();

        let base = WidgetVisuals {
            bg_fill: SECONDARY,
            bg_stroke: Stroke {
                color: GREY_WHITE,
                width: 0.,
            },
            rounding: Rounding {
                sw: 5.,
                ne: 5.,
                nw: 5.,
                se: 5.,
            },
            expansion: 1.,
            fg_stroke: Stroke {
                color: GREY_WHITE,
                width: 1.,
            },
        };

        style.visuals.widgets.inactive = base;
        style.visuals.widgets.active = base;

        style.visuals.widgets.open = WidgetVisuals {
            bg_stroke: Stroke {
                color: GREY_WHITE,
                width: 1.,
            },
            ..base
        };
        style.visuals.widgets.noninteractive = WidgetVisuals {
            bg_fill: SECONDARY_BRIGHT,
            ..base
        };

        style.visuals.widgets.hovered = WidgetVisuals {
            bg_fill: SECONDARY_DARK,
            ..base
        };

        style.visuals.selection = Selection {
            bg_fill: SECONDARY_DARK,
            stroke: Stroke {
                color: GREY_WHITE,
                width: 1.,
            },
        };

        style.visuals.extreme_bg_color = SECONDARY;
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
            render_header(ui, logo);
            render_draw_line(ui, 2.);

            render_section(ui, "Styling", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Background:");

                    ComboBox::from_id_source("background_options")
                        .selected_text(format!("{:?}", self.conf.background))
                        .show_ui(ui, |ui| {
                            for option in BackgroundOptions::iter().collect::<Vec<_>>() {
                                ui.selectable_value(
                                    &mut self.conf.background,
                                    option,
                                    format!("{:?}", option),
                                );
                            }
                        });
                });

                ui.add_space(PADDING);

                match self.conf.background {
                    BackgroundOptions::Solid => {
                        ui.horizontal(|ui| {
                            ui.label("Pick a Color:");
                            ui.color_edit_button_srgb(&mut self.conf.bg_color);
                        });
                    }
                    BackgroundOptions::FromURL => {
                        ui.horizontal(|ui| {
                            ui.label("Image URL:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.conf.bg_url)
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
                            if ui.button("Open file…").clicked() {
                                if let Some(path) = rfd::FileDialog::new().pick_file() {
                                    let location = path.display().to_string();
                                    let file_name = get_file_name_from_path(&location);
                                    let supported_file_ext = ["png", "gif", "jpg", "jpeg"];
                                    let file_ext =
                                        file_name.split(".").collect::<Vec<&str>>().pop().unwrap();

                                    if supported_file_ext.contains(&file_ext) {
                                        self.invalid_bg = false;
                                        self.conf.bg_location = location;
                                    } else {
                                        self.invalid_bg = true;
                                    }
                                }
                            }

                            if self.invalid_bg {
                                ui.colored_label(Color32::from_rgb(255, 48, 48), "Not an Image");
                            } else if !self.conf.bg_location.is_empty() {
                                ui.colored_label(
                                    Color32::from_rgba_unmultiplied(254, 216, 67, 200),
                                    get_file_name_from_path(&self.conf.bg_location),
                                );
                            }
                        });
                    }
                }

                if self.conf.background == BackgroundOptions::FromDisk
                    || self.conf.background == BackgroundOptions::FromURL
                {
                    ui.add_space(PADDING);

                    ui.horizontal(|ui| {
                        ui.label("Wallpaper Mode:");

                        ComboBox::from_id_source("background_mode")
                            .selected_text(format!("{:?}", self.conf.wallpaper_mode))
                            .show_ui(ui, |ui| {
                                for option in WallpaperMode::iter().collect::<Vec<_>>() {
                                    ui.selectable_value(
                                        &mut self.conf.wallpaper_mode,
                                        option,
                                        format!("{:?}", option),
                                    );
                                }
                            });
                    });
                }

                ui.add_space(PADDING);

                ui.horizontal(|ui| {
                    ui.label("Update every:");

                    ComboBox::from_id_source("update_every")
                        .selected_text(format!("{:?}", self.conf.update_every))
                        .show_ui(ui, |ui| {
                            for option in UpdateEvery::iter().collect::<Vec<_>>() {
                                ui.selectable_value(
                                    &mut self.conf.update_every,
                                    option,
                                    format!("{:?}", option),
                                );
                            }
                        });
                });

                ui.add_space(PADDING);

                ui.horizontal(|ui| {
                    ui.label("Font:");

                    ComboBox::from_id_source("font_family")
                        .width(125.)
                        .selected_text(format!("{:?}", self.conf.font))
                        .show_ui(ui, |ui| {
                            for option in Font::iter().collect::<Vec<_>>() {
                                ui.selectable_value(
                                    &mut self.conf.font,
                                    option,
                                    format!("{:?}", option),
                                );
                            }
                        });
                });

                ui.add_space(PADDING);

                ui.horizontal(|ui| {
                    ui.label("Font Size:");
                    ui.add(egui::Slider::new(&mut self.conf.font_size, 5..=255));
                });

                ui.add_space(PADDING);

                ui.horizontal(|ui| {
                    ui.label("Font Color:");
                    ui.color_edit_button_srgb(&mut self.conf.font_color);
                });
            });

            render_section(ui, "Pick your Deadline", |ui| {
                let date_error_popup_id = ui.make_persistent_id("invalid-date-error");

                render_input_with_label(ui, "Date:", &mut self.conf.date, "2022-08-26");

                ui.add_space(PADDING);

                ui.horizontal(|ui| {
                    ui.label("Time:");

                    render_input(ui, &mut self.conf.hours, "7", 18.);
                    ui.label(":");
                    render_input(ui, &mut self.conf.minutes, "28", 18.);

                    // Check if inputs are numeric
                    if !is_string_numeric(&self.conf.hours) {
                        self.conf.hours = String::new();
                    }
                    if !is_string_numeric(&self.conf.minutes) {
                        self.conf.minutes = String::new();
                    }

                    ComboBox::from_id_source("time_period")
                        .width(70.)
                        .selected_text(format!("{:?}", self.conf.period))
                        .show_ui(ui, |ui| {
                            for option in Periods::iter().collect::<Vec<_>>() {
                                ui.selectable_value(
                                    &mut self.conf.period,
                                    option,
                                    format!("{:?}", option),
                                );
                            }
                        });
                });

                ui.add_space(20.);

                let button = button("Start!");
                let button = ui.add(button);

                // Setup error popup
                egui::popup::popup_below_widget(ui, date_error_popup_id, &button, |ui| {
                    ui.set_min_width(200.0); // if you want to control the size
                    ui.label(&self.error_msg);
                });

                if button.clicked() {
                    match sanitize_inputs(&self.conf) {
                        Err(msg) => {
                            self.error_msg = msg;
                            ui.memory().toggle_popup(date_error_popup_id);
                        }
                        _ => (),
                    }
                };
            });
        });
    }

    fn name(&self) -> &str {
        "Deadliner"
    }
}

impl<'a> Deadliner<'a> {
    pub fn new() -> Deadliner<'a> {
        Deadliner {
            textures: HashMap::new(),
            error_msg: String::new(),
            invalid_bg: false,
            screen: (0, 0),
            conf: DeadlinerConf {
                background: BackgroundOptions::Solid,
                bg_color: [0, 0, 0],
                bg_location: String::new(),
                bg_url: String::new(),
                update_every: UpdateEvery::Hour,
                font: Font::PoppinsBlack,
                date: String::new(),
                hours: String::new(),
                minutes: String::new(),
                period: Periods::AM,
                font_size: 100,
                font_color: [255, 255, 255],
                wallpaper_mode: WallpaperMode::Center,
            },
        }
    }

    fn set_custom_fonts(&mut self, ctx: &Context) {
        let mut fonts = FontDefinitions::default();
        let fonts_data: Vec<(&str, u16, Vec<u8>)> = vec![
            (
                "Poppins-Regular",
                400,
                fs::read("./assets/fonts/PoppinsLight.ttf").unwrap(),
            ),
            (
                "Poppins-Medium",
                500,
                fs::read("./assets/fonts/PoppinsRegular.ttf").unwrap(),
            ),
            (
                "Poppins-SemiBold",
                600,
                fs::read("./assets/fonts/PoppinsMedium.ttf").unwrap(),
            ),
        ];

        // Insert all of the fonts data
        for (name, font_weight, buffer) in fonts_data {
            fonts
                .font_data
                .insert(name.to_owned(), FontData::from_owned(buffer));

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
                family: FontFamily::Name("Poppins-600".into()),
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

        text_styles.insert(
            TextStyle::Monospace,
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
