use crate::{
    draw_line, is_string_numeric, update_wallpaper, BACKGROUND, BLACK, GREY_WHITE,
    INPUT_BACKGROUND, MARGIN, PADDING, WHITE, YELLOW,
};
use chrono::{
    Date, DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, ParseError, TimeZone, Utc,
};
use eframe::{
    self,
    egui::{
        self, style::Margin, text, CentralPanel, ComboBox, Context, FontData, FontDefinitions,
        Frame, Id, RichText, TextStyle,
    },
    epaint::{Color32, FontFamily, FontId, TextureHandle},
    epi::App,
};
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    str::FromStr,
    thread,
    time::Instant,
};

struct DeadlinerConf {
    background: BackgroundOptions,

    bg_color: [u8; 3],
    bg_url: String,
    bg_location: String,

    update_every: UpdateEvery,

    font: Font,
    font_size: u8,
    font_color: [u8; 3],

    date: String,

    hours: String,

    minutes: String,

    period: Periods,
}

pub struct Deadliner<'a> {
    // Preloaded textures on setup to use in the lifecycle methods.
    textures: HashMap<&'a str, TextureHandle>,

    error_msg: String,

    conf: DeadlinerConf,
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

#[derive(PartialEq, Debug, Clone, Copy)]
enum Font {
    PoppinsBlack,
    PoppinsMedium,
    PoppinsRegular,
    PoppinsLight,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum UpdateEvery {
    Day,
    Hour,
    Minute,
}

#[derive(Debug)]
pub struct SanitizedConf {
    bg_color: Option<String>,
    bg_url: Option<String>,
    bg_location: Option<String>,

    update_every: UpdateEvery,
    font: Font,
    font_size: u8,
    font_color: String,

    deadline: NaiveDateTime,
}

fn sanitize_inputs(conf: &DeadlinerConf) -> Result<(), String> {
    if conf.date.is_empty() || conf.hours.is_empty() || conf.minutes.is_empty() {
        return Err(String::from("Not enough Date Inputs"));
    }

    let mut sanitized_conf = SanitizedConf {
        bg_color: None,
        bg_url: None,
        bg_location: None,
        update_every: conf.update_every,
        font: conf.font,
        font_size: conf.font_size,

        // Just a placeholder till we convert RGB to HEX
        font_color: String::new(),

        // Just a placeholder till we parse the date
        deadline: NaiveDateTime::from_timestamp(0, 0),
    };

    let rgb_to_hex = |r, g, b| format!("#{:02X}{:02X}{:02X}", r, g, b);
    let [r, g, b] = conf.bg_color;

    // font-color RGB to HEX
    {
        let [r, g, b] = conf.font_color;

        sanitized_conf.font_color = rgb_to_hex(r, g, b);
    }

    // bg-color RGB to HEX
    match conf.background {
        BackgroundOptions::Solid => sanitized_conf.bg_color = Some(rgb_to_hex(r, g, b)),
        BackgroundOptions::FromDisk => {
            sanitized_conf.bg_location = Some(conf.bg_location.trim().to_string())
        }
        BackgroundOptions::FromURL => sanitized_conf.bg_url = Some(conf.bg_url.trim().to_string()),
    }

    let formatted_date_str = format!(
        "{} {}:{} {:?}",
        conf.date.trim(),
        conf.hours.trim().to_string(),
        conf.minutes.trim(),
        conf.period
    );

    let date = NaiveDateTime::parse_from_str(&formatted_date_str, "%Y-%m-%d %I:%M %p");

    match date {
        Ok(result) => sanitized_conf.deadline = result,
        Err(_) => return Err(String::from("Invalid date input!")),
    }

    let today = Local::now().naive_local();
    let deadline = date.unwrap();
    let diff = deadline.signed_duration_since(today);

    let days = diff.num_days();
    let hours = diff.num_hours();

    // TODO: approximate values
    // Ex: 1 hour and 31 minutes
    // Should be "2 hours remaining"
    // And not "1 hours remaining"

    let remaining_days = days;
    let remaining_hours = hours - remaining_days * 24;

    thread::spawn(move || {
        // Here we setup a schedule every "period" to update the wallpaper
        update_wallpaper(
            &format!("{} Days, {} Hours Left.", remaining_days, remaining_hours),
            sanitized_conf.font_size,
            sanitized_conf.font_color,
        );
    });

    Ok(())
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
                    .selected_text(format!("{:?}", self.conf.background))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.conf.background,
                            BackgroundOptions::Solid,
                            "Solid Color",
                        );
                        ui.selectable_value(
                            &mut self.conf.background,
                            BackgroundOptions::FromDisk,
                            "From Disk",
                        );
                        ui.selectable_value(
                            &mut self.conf.background,
                            BackgroundOptions::FromURL,
                            "From URL",
                        );
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
                        ui.label("Image Location:");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.conf.bg_location)
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
                    .selected_text(format!("{:?}", self.conf.update_every))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.conf.update_every, UpdateEvery::Day, "Day");
                        ui.selectable_value(&mut self.conf.update_every, UpdateEvery::Hour, "Hour");
                        ui.selectable_value(
                            &mut self.conf.update_every,
                            UpdateEvery::Minute,
                            "Minute",
                        );
                    });
            });

            ui.add_space(PADDING);

            ui.horizontal(|ui| {
                ui.label("Font:");

                ComboBox::from_id_source("font_family")
                    .selected_text(format!("{:?}", self.conf.font))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.conf.font,
                            Font::PoppinsBlack,
                            "Poppins Black",
                        );
                        ui.selectable_value(
                            &mut self.conf.font,
                            Font::PoppinsMedium,
                            "Poppins Medium",
                        );
                        ui.selectable_value(
                            &mut self.conf.font,
                            Font::PoppinsRegular,
                            "Poppins Regular",
                        );
                        ui.selectable_value(
                            &mut self.conf.font,
                            Font::PoppinsLight,
                            "Poppins Light",
                        );
                    });
            });

            ui.add_space(PADDING);

            ui.horizontal(|ui| {
                ui.label("Font Size:");
                ui.add(egui::Slider::new(&mut self.conf.font_size, 0..=250));
            });

            ui.add_space(PADDING);

            ui.horizontal(|ui| {
                ui.label("Font Color:");
                ui.color_edit_button_srgb(&mut self.conf.font_color);
            });

            ui.add_space(PADDING);

            ui.heading("Pick your Deadline");

            ui.add_space(PADDING);

            let date_error_popup_id = ui.make_persistent_id("invalid-date-error");

            ui.horizontal_wrapped(|ui| {
                ui.label("Date:");

                ui.add(
                    egui::TextEdit::singleline(&mut self.conf.date)
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
                    egui::TextEdit::singleline(&mut self.conf.hours)
                        .desired_width(18.)
                        .hint_text(RichText::new("7").color(Color32::from_white_alpha(45))),
                );

                ui.label(":");
                ui.add(
                    egui::TextEdit::singleline(&mut self.conf.minutes)
                        .desired_width(18.)
                        .hint_text(RichText::new("28").color(Color32::from_white_alpha(45))),
                );

                // Check if inputs are numeric
                if !is_string_numeric(&self.conf.hours) {
                    self.conf.hours = String::new();
                }
                if !is_string_numeric(&self.conf.minutes) {
                    self.conf.minutes = String::new();
                }

                ComboBox::from_id_source("time_period")
                    .selected_text(format!("{:?}", self.conf.period))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.conf.period, Periods::AM, "AM");
                        ui.selectable_value(&mut self.conf.period, Periods::PM, "PM");
                    });
            });

            ui.add_space(20.);

            let button = ui.button(
                RichText::new("Start!")
                    .font(FontId {
                        family: FontFamily::Name("Poppins-600".into()),
                        size: 35.,
                    })
                    .background_color(YELLOW)
                    .color(BLACK),
            );

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
            error_msg: String::new(),
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
            },
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
