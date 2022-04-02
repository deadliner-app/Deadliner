use std::fs;

use crate::{
    download_image, get_cache_dir, new_path, unwrap_or_return, BackgroundOptions, Font,
    SanitizedConf, ScreenDimensions,
};
use chrono::{Local, NaiveDateTime};
use image::{imageops::FilterType, DynamicImage, Rgb, RgbImage};
use imageproc::{
    drawing::{draw_filled_rect_mut, Canvas},
    rect::Rect,
};
use text_to_png::TextRenderer;

pub fn update_wallpaper(conf: &SanitizedConf, test_text_dimensions: bool) -> Result<(), String> {
    let today = Local::now().naive_local();
    let deadline = NaiveDateTime::parse_from_str(&conf.deadline_str, "%Y-%m-%d %I:%M %p").unwrap();
    let diff = deadline.signed_duration_since(today);

    let remaining_days = diff.num_days();
    let months = remaining_days / 30;
    let mut weeks = remaining_days / 7;
    let mut days = remaining_days;
    let mut hours = diff.num_hours();
    let minutes = diff.num_minutes();

    if conf.show_months {
        // Month = 30 days - Month = 4 Weeks = 28 days
        // Reminder is 2 days from the 4 weeks of each month
        let days_in_months = months * 30;
        let weeks_in_months = days_in_months / 7;
        let days_reminder = days_in_months - weeks_in_months * 7;

        weeks = weeks - weeks_in_months;
        days = days - months * (30 - days_reminder);
        hours = hours - months * (30 - days_reminder) * 24;
    }

    if conf.show_weeks {
        days = days - weeks * 7;
        hours = hours - weeks * 7 * 24;
    }

    if conf.show_days {
        hours = hours - days * 24
    }

    // TODO: approximate values
    // Ex: 1 hour and 31 minutes
    // Should be "2 hours remaining"
    // And not "1 hours remaining"

    let mut deadline_str = String::new();

    let show_months = conf.show_months && (months != 0 || test_text_dimensions);
    let show_weeks = conf.show_weeks && (weeks != 0 || test_text_dimensions && months > 1);
    let show_days = conf.show_days && (days != 0 || test_text_dimensions && weeks > 1);
    let show_hours = conf.show_hours && (hours != 0 || test_text_dimensions && days > 1);

    let format_time_unit =
        |time_unit, num| format!("{} {}{}", num, time_unit, if num > 1 { "s" } else { "" });

    if show_months {
        deadline_str.push_str(&format_time_unit("Month", months));
    }

    if show_weeks {
        if show_months {
            deadline_str.push_str(", ");
        }
        deadline_str.push_str(&format_time_unit("Week", weeks));
    }

    if show_days {
        if show_months || show_weeks {
            deadline_str.push_str(", ");
        }
        deadline_str.push_str(&format_time_unit("Day", days));
    }

    if show_hours {
        if show_months || show_weeks || show_days {
            deadline_str.push_str(", ");
        }
        deadline_str.push_str(&format_time_unit("Hour", hours));
    }

    // If the deadline is close to its ending, show minutes left.
    if deadline_str.is_empty() && conf.show_hours {
        deadline_str.push_str(&format_time_unit("Minute", minutes));
    }

    deadline_str.push_str(" Left.");

    // TODO: Prevent blocking the main thread cause it freezes the UI.
    let file_path = generate_wallpaper(&deadline_str, &conf);

    match file_path {
        Ok(file_path) => {
            if !test_text_dimensions {
                // Sets the wallpaper for the current desktop from a URL.
                wallpaper::set_mode(conf.bg_mode.into()).unwrap();
                wallpaper::set_from_path(&file_path).unwrap();
            }
            Ok(())
        }
        Err(msg) => Err(msg),
    }
}

pub fn generate_wallpaper(deadline_str: &str, conf: &SanitizedConf) -> Result<String, String> {
    let font_date_bytes = if conf.font == Font::ChooseFromDisk {
        fs::read(&conf.custom_font_location).unwrap()
    } else {
        fs::read(new_path(&format!("assets/fonts/{:?}.ttf", conf.font))).unwrap()
    };

    let renderer = TextRenderer::try_new_with_ttf_font_data(font_date_bytes).unwrap();

    let text_png = renderer
        .render_text_to_png_data(deadline_str, conf.font_size, conf.font_color.as_str())
        .unwrap();

    let text_image = image::load_from_memory(&text_png.data).unwrap();

    let mut background;

    if conf.bg_type == BackgroundOptions::FromDisk {
        background = image::open(conf.bg_location.as_ref().unwrap()).unwrap();
    } else if conf.bg_type == BackgroundOptions::Solid {
        let ScreenDimensions { width, height } = conf.screen_dimensions;

        let mut image = RgbImage::new(width, height);

        draw_filled_rect_mut(
            &mut image,
            Rect::at(0, 0).of_size(width, height),
            Rgb(conf.bg_color_arr),
        );

        background = DynamicImage::ImageRgb8(image);
    } else {
        let downloaded_image = match download_image(conf.bg_url.as_ref().unwrap()) {
            Ok(img) => img,
            Err(_) => {
                return Err(String::from(
                    "Couldn't download the Image from the supplied URL!",
                ))
            }
        };

        background = image::io::Reader::open(downloaded_image)
            .unwrap()
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
    }

    if background.width() <= text_png.size.width || background.height() <= text_png.size.height {
        return Err(String::from(
            "Font size is bigger than wallpaper's dimensions!",
        ));
    }

    // 50% Background Image width or height - 50% Text Image width or height
    // To Center the text both horizontally and vertically
    let x = background.width() / 2 - text_png.size.width / 2;
    let y = background.height() / 2 - text_png.size.height / 2;

    image::imageops::overlay(&mut background, &text_image, x, y);

    let file_path = get_cache_dir().join("result.png");
    let file_path = file_path.to_str().unwrap().to_owned();

    unwrap_or_return!(background.save(&file_path), "Couldn't save result.png");

    Ok(file_path)
}

pub fn generate_deadline_over_wallpaper(
    deadline_str: &str,
    conf: &SanitizedConf,
) -> Result<String, String> {
    let font_date_bytes = fs::read(new_path(&format!("assets/fonts/{:?}.ttf", conf.font))).unwrap();

    let renderer = TextRenderer::try_new_with_ttf_font_data(font_date_bytes).unwrap();

    let text_png = renderer
        .render_text_to_png_data(deadline_str, conf.font_size, conf.font_color.as_str())
        .unwrap();

    let text_image = image::load_from_memory(&text_png.data).unwrap();

    let mut background;

    if conf.bg_type == BackgroundOptions::FromDisk {
        background = image::open(conf.bg_location.as_ref().unwrap()).unwrap();
    } else if conf.bg_type == BackgroundOptions::Solid {
        let ScreenDimensions { width, height } = conf.screen_dimensions;

        let mut image = RgbImage::new(width, height);

        draw_filled_rect_mut(
            &mut image,
            Rect::at(0, 0).of_size(width, height),
            Rgb(conf.bg_color_arr),
        );

        background = DynamicImage::ImageRgb8(image);
    } else {
        let downloaded_image = match download_image(conf.bg_url.as_ref().unwrap()) {
            Ok(img) => img,
            Err(_) => {
                return Err(String::from(
                    "Couldn't download the Image from the supplied URL!",
                ))
            }
        };

        background = image::io::Reader::open(downloaded_image)
            .unwrap()
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
    }

    if background.width() <= text_png.size.width || background.height() <= text_png.size.height {
        return Err(String::from(
            "Font size is bigger than wallpaper's dimensions!",
        ));
    }

    // 50% Background Image width or height - 50% Text Image width or height
    // To Center the text both horizontally and vertically
    let x = background.width() / 2 - text_png.size.width / 2;
    let y = background.height() / 2 - text_png.size.height / 2;

    let party_popper = image::open(new_path("assets/party-popper.png")).unwrap();
    let party_popper_size = {
        let size = (background.width() as f64 * 0.1) as u32;
        if size > party_popper.width() {
            party_popper.width()
        } else {
            size
        }
    };

    let party_popper = image::imageops::resize(
        &party_popper,
        party_popper_size,
        party_popper_size,
        FilterType::Nearest,
    );

    image::imageops::overlay(&mut background, &text_image, x, y);

    // Add party poppers left and right.
    let offset = 30;
    image::imageops::overlay(
        &mut background,
        &party_popper,
        offset,
        conf.screen_dimensions.height - party_popper.height() - offset,
    );
    image::imageops::overlay(
        &mut background,
        &image::imageops::flip_horizontal(&party_popper),
        conf.screen_dimensions.width - party_popper.width() - offset,
        conf.screen_dimensions.height - party_popper.height() - offset,
    );

    let file_path = get_cache_dir().join("result.png");
    let file_path = file_path.to_str().unwrap().to_owned();

    unwrap_or_return!(background.save(&file_path), "Couldn't save result.png");

    Ok(file_path)
}
