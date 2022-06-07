mod components;
mod deadliner;
mod design_system;
mod macros;
mod short_hash;
mod update_wallpaper;

pub use components::*;
pub use deadliner::*;
pub use design_system::*;
pub use macros::*;
use serde::{Deserialize, Serialize};
pub use short_hash::*;
pub use update_wallpaper::*;

use chrono::{Local, NaiveDateTime};
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use std::{env, fs, thread};
use std::{fs::File, path};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SanitizedConf {
    pub screen_dimensions: ScreenDimensions,

    pub default_bg: SanitizedBackground,
    pub bg_mode: WallpaperMode,

    pub show_months: bool,
    pub show_weeks: bool,
    pub show_days: bool,
    pub show_hours: bool,

    pub font: Font,
    pub font_size: u8,
    pub font_color: String,
    pub custom_font_location: String,

    pub deadline_str: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SanitizedBackground {
    Solid { rgb: [u8; 3], hex: String },
    FromDisk(String),
    FromURL(String),
}

impl From<Background> for SanitizedBackground {
    fn from(b: Background) -> Self {
        match b {
            Background::Solid([r, g, b]) => Self::Solid {
                rgb: [r, g, b],
                hex: rgb_to_hex(r, g, b),
            },
            Background::FromDisk { location, .. } => Self::FromDisk(location.trim().to_string()),
            Background::FromURL { url, .. } => Self::FromURL(url.trim().to_string()),
        }
    }
}

fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn save_inputs(conf: &DeadlinerConf) -> Result<(), String> {
    if !(conf.show_months || conf.show_weeks || conf.show_days || conf.show_hours)
        || conf.date.is_empty()
        || conf.hours.is_empty()
        || conf.minutes.is_empty()
    {
        return Err(String::from("Not enough Inputs"));
    }

    let mut sanitized_conf = SanitizedConf {
        screen_dimensions: conf.screen_dimensions.clone(),
        default_bg: conf.default_background.clone().into(),
        bg_mode: conf.default_background.mode(),
        font: conf.font,
        font_size: conf.font_size,

        show_months: conf.show_months,
        show_weeks: conf.show_weeks,
        show_days: conf.show_days,
        show_hours: conf.show_hours,
        custom_font_location: conf.custom_font_location.clone(),

        // Just a placeholder till we convert RGB to HEX
        font_color: String::new(),

        // Just a placeholder till we parse the date
        deadline_str: String::new(),
    };

    // font-color RGB to HEX
    {
        let [r, g, b] = conf.font_color;

        sanitized_conf.font_color = rgb_to_hex(r, g, b);
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
        Ok(_) => sanitized_conf.deadline_str = formatted_date_str,
        Err(_) => return Err(String::from("Invalid date input!")),
    }

    // Check if deadline was in the future
    let today = Local::now().naive_local();
    let deadline =
        NaiveDateTime::parse_from_str(&sanitized_conf.deadline_str, "%Y-%m-%d %I:%M %p").unwrap();
    let diff = deadline.signed_duration_since(today);

    let minutes = diff.num_minutes();
    if minutes <= 0 {
        return Err(String::from("Deadline must be a future date!"));
    }

    // Run update_wallpaper once to check for any potential errors before saving this conf.
    update_wallpaper(&sanitized_conf, true)?;

    update_wallpaper(&sanitized_conf, false)?;

    // If we managed to update the wallpaper successfully, then save the current conf.
    // Write the config.json next to the binaries instead of in the cache dir cause this is a very
    // important file. And it would be bad if it was accidently deleted when the cache was cleared
    unwrap_or_return!(
        fs::write(
            new_path("config.json"),
            serde_json::to_string_pretty(&sanitized_conf).unwrap(),
        ),
        "Couldn't save your configuration to the filesystem!"
    );

    let cache_conf = get_cache_dir().join("raw_config.json");

    unwrap_or_return!(
        fs::write(cache_conf, serde_json::to_string_pretty(&conf).unwrap(),),
        "Couldn't save your configuration to the filesystem!"
    );

    // Check if there's an already running instance of schedular
    let port = fs::read_to_string(new_path("port.txt")).unwrap();
    let server_url = format!("http://127.0.0.1:{}", port);

    let res = reqwest::blocking::Client::builder()
        .timeout(Duration::from_millis(50))
        .build()
        .unwrap()
        .get(&server_url)
        .send();

    let is_schedular_running = res.is_ok() && res.unwrap().status().as_u16() == 200;

    if is_schedular_running {
        // Send a request to shutdown the running schedular
        reqwest::blocking::get(server_url + "/shutdown").unwrap();

        // Give the schedular a bit of time till it shutdown
        thread::sleep(Duration::from_millis(50));
    }

    // !Here we setup a schedule to update the wallpaper
    let schedular_exec = format!("deadliner-schedular{}", &get_current_file_ext());
    unwrap_or_return!(
        Command::new(new_path(&schedular_exec))
            .arg("skip-update-on-launch")
            .spawn(),
        "Couldn't run the schedular binary!"
    );

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

pub fn get_file_name_from_path(file_path: &str) -> String {
    let location_paths: Vec<&str> = file_path.split(path::MAIN_SEPARATOR).collect();
    let file_name = location_paths[location_paths.len() - 1];

    file_name.to_string()
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

pub fn new_path(path: &str) -> PathBuf {
    let mut exe_location = env::current_exe().unwrap();

    exe_location.pop();

    exe_location.join(path)
}

pub fn get_cache_dir() -> PathBuf {
    let cache_dir = dirs::cache_dir().ok_or("no cache dir").unwrap();

    cache_dir.join("deadliner")
}

pub fn get_current_file_ext() -> String {
    let curr_exe = std::env::current_exe().unwrap();
    let splitted_by_dots = curr_exe
        .as_os_str()
        .to_str()
        .unwrap()
        .split(".")
        .collect::<Vec<&str>>();

    if splitted_by_dots.len() < 2 {
        String::new()
    } else {
        ".".to_string() + splitted_by_dots[splitted_by_dots.len() - 1]
    }
}
