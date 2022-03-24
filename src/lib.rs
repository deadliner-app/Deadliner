mod components;
mod deadliner;
mod design_system;
mod update_wallpaper;

use chrono::NaiveDateTime;
pub use components::*;
pub use deadliner::*;
pub use design_system::*;
pub use update_wallpaper::*;

use std::{path, thread};

fn sanitize_inputs(conf: &DeadlinerConf) -> Result<(), String> {
    if conf.date.is_empty() || conf.hours.is_empty() || conf.minutes.is_empty() {
        return Err(String::from("Not enough Inputs"));
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

    thread::spawn(move || {
        // Here we setup a schedule every "period" to update the wallpaper
        update_wallpaper(sanitized_conf);
    });

    Ok(())
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
