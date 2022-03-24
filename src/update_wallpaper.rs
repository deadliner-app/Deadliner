use std::fs;

use chrono::Local;
use image::{DynamicImage, Rgb, RgbImage};
use imageproc::{
    drawing::{draw_filled_rect_mut, Canvas},
    rect::Rect,
};
use text_to_png::TextRenderer;

use crate::{download_image, BackgroundOptions, SanitizedConf};

pub fn update_wallpaper(conf: SanitizedConf) -> Result<(), String> {
    let today = Local::now().naive_local();
    let deadline = conf.deadline;
    let diff = deadline.signed_duration_since(today);

    let minutes = diff.num_minutes();
    let days = diff.num_days();
    let hours = diff.num_hours();

    // TODO: approximate values
    // Ex: 1 hour and 31 minutes
    // Should be "2 hours remaining"
    // And not "1 hours remaining"

    let remaining_days = days;
    let remaining_hours = hours - remaining_days * 24;

    if minutes <= 0 {
        return Err(String::from("Deadline must be a future date!"));
    }

    let deadline_str = format!("{} Days, {} Hours Left.", remaining_days, remaining_hours);

    // TODO: Prevent blocking the main thread cause it freezes the UI.
    let file_path = generate_wallpaper(&deadline_str, &conf);

    match file_path {
        Ok(file_path) => {
            // Sets the wallpaper for the current desktop from a URL.
            wallpaper::set_mode(conf.bg_mode).unwrap();
            wallpaper::set_from_path(&file_path).unwrap();

            Ok(())
        }
        Err(msg) => Err(msg),
    }
}

fn generate_wallpaper(deadline_str: &str, conf: &SanitizedConf) -> Result<String, String> {
    let font_date_bytes = fs::read(&format!("./assets/fonts/{:?}.ttf", conf.font)).unwrap();

    let renderer = TextRenderer::try_new_with_ttf_font_data(font_date_bytes).unwrap();

    let text_png = renderer
        .render_text_to_png_data(deadline_str, conf.font_size, conf.font_color.as_str())
        .unwrap();

    let text_image = image::load_from_memory(&text_png.data).unwrap();

    let mut background;

    if conf.bg_type == BackgroundOptions::FromDisk {
        background = image::open(conf.bg_location.as_ref().unwrap()).unwrap();
    } else if conf.bg_type == BackgroundOptions::Solid {
        let mut image = RgbImage::new(1920, 1080);

        draw_filled_rect_mut(
            &mut image,
            Rect::at(0, 0).of_size(1920, 1080),
            Rgb(conf.bg_color_arr),
        );

        background = DynamicImage::ImageRgb8(image);
    } else {
        let downloaded_image = download_image(conf.bg_url.as_ref().unwrap()).unwrap();

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

    let cache_dir = dirs::cache_dir().ok_or("no cache dir").unwrap();
    let file_path = cache_dir.join("./deadliner/result.png");
    let file_path = file_path.to_str().unwrap().to_owned();

    match background.save(&file_path) {
        Err(_) => return Err(String::from("Couldn't save result.png")),
        _ => {}
    }

    Ok(file_path)
}
