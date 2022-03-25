mod components;
mod deadliner;
mod design_system;
mod short_hash;
mod update_wallpaper;

pub use components::*;
pub use deadliner::*;
pub use design_system::*;
pub use short_hash::*;
pub use update_wallpaper::*;

use chrono::NaiveDateTime;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::{fs::File, path};
use wallpaper::Mode;

#[derive(Debug)]
pub struct SanitizedConf {
    bg_type: BackgroundOptions,
    bg_color: Option<String>,
    bg_color_arr: [u8; 3],
    bg_url: Option<String>,
    bg_location: Option<String>,

    bg_mode: Mode,

    show_months: bool,
    show_weeks: bool,
    show_days: bool,
    show_hours: bool,

    font: Font,
    font_size: u8,
    font_color: String,

    deadline: NaiveDateTime,
}

fn sanitize_inputs(conf: &DeadlinerConf) -> Result<(), String> {
    if !(conf.show_months || conf.show_weeks || conf.show_days || conf.show_hours)
        || conf.date.is_empty()
        || conf.hours.is_empty()
        || conf.minutes.is_empty()
    {
        return Err(String::from("Not enough Inputs"));
    }

    let mut sanitized_conf = SanitizedConf {
        bg_mode: map_wallpaper_mode_enum(conf.wallpaper_mode),
        bg_color: None,
        bg_color_arr: conf.bg_color,
        bg_url: None,
        bg_location: None,
        font: conf.font,
        font_size: conf.font_size,
        bg_type: conf.background,

        show_months: conf.show_months,
        show_weeks: conf.show_weeks,
        show_days: conf.show_days,
        show_hours: conf.show_hours,

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

    // Here we setup a schedule every "period" to update the wallpaper
    update_wallpaper(sanitized_conf)
}

pub fn is_string_numeric(word: &str) -> bool {
    for c in word.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}

pub fn get_file_name_from_path(file_path: &str) -> &str {
    let location_paths: Vec<&str> = file_path.split(path::MAIN_SEPARATOR).collect();
    let file_name = location_paths[location_paths.len() - 1];

    file_name
}

type DownloadResult<T> = std::result::Result<T, Box<dyn Error>>;

pub fn download_image(url: &str) -> DownloadResult<String> {
    let cache_dir = dirs::cache_dir().ok_or("no cache dir")?;
    let file_path = cache_dir.join(format!("./deadliner/{}.png", unique_hash(url)));

    if !file_path.exists() {
        let mut file = File::create(&file_path)?;
        reqwest::blocking::get(url)?.copy_to(&mut file)?;
    }

    Ok(file_path.to_str().to_owned().ok_or("no file path")?.into())
}

pub fn map_wallpaper_mode_enum(mode: WallpaperMode) -> Mode {
    match mode {
        WallpaperMode::Center => Mode::Center,
        WallpaperMode::Crop => Mode::Crop,
        WallpaperMode::Fit => Mode::Fit,
        WallpaperMode::Span => Mode::Span,
    }
}

pub fn new_path(path: &str) -> PathBuf {
    let mut exe_location = env::current_exe().unwrap();

    exe_location.pop();

    exe_location.join(path)
}
