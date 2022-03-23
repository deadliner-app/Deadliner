use std::fs;

use chrono::Local;
use text_to_png::TextRenderer;

use crate::SanitizedConf;

pub fn update_wallpaper(conf: SanitizedConf) {
    let today = Local::now().naive_local();
    let deadline = conf.deadline;
    let diff = deadline.signed_duration_since(today);

    let days = diff.num_days();
    let hours = diff.num_hours();

    // TODO: approximate values
    // Ex: 1 hour and 31 minutes
    // Should be "2 hours remaining"
    // And not "1 hours remaining"

    let remaining_days = days;
    let remaining_hours = hours - remaining_days * 24;

    let deadline_str = format!("{} Days, {} Hours Left.", remaining_days, remaining_hours);

    let file_path = generate_wallpaper(&deadline_str, conf);

    // Sets the wallpaper for the current desktop from a URL.
    wallpaper::set_mode(wallpaper::Mode::Center).unwrap();
    wallpaper::set_from_path(&file_path).unwrap();
    // Returns the wallpaper of the current desktop.
    println!("{:?}", wallpaper::get());
}

fn generate_wallpaper(deadline_str: &str, conf: SanitizedConf) -> String {
    let font_date_bytes = fs::read(&format!("./assets/fonts/{:?}.ttf", conf.font)).unwrap();

    let renderer = TextRenderer::try_new_with_ttf_font_data(font_date_bytes).unwrap();

    let text_png = renderer
        .render_text_to_png_data(deadline_str, conf.font_size, conf.font_color.as_str())
        .unwrap();

    let text_image = image::load_from_memory(&text_png.data).unwrap();

    let mut background = image::open("./assets/background.png").unwrap();

    // 50% Background Image width or height - 50% Text Image width or height
    // To Center the text both horizontally and vertically
    let x = background.width() / 2 - text_png.size.width / 2;
    let y = background.height() / 2 - text_png.size.height / 2;

    image::imageops::overlay(&mut background, &text_image, x as i64, y as i64);

    let cache_dir = dirs::cache_dir().ok_or("no cache dir").unwrap();
    let file_path = cache_dir.join("result.png");
    let file_path = file_path.to_str().unwrap().to_owned();

    background
        .save(&file_path)
        .expect("Couldn't save result.png");

    file_path
}
