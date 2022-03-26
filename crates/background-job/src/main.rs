use deadliner_gui::{new_path, SanitizedConf};
use std::fs;

fn main() {
    let conf_str =
        fs::read_to_string(new_path("config.json")).expect("Can't read Config JSON file!");

    let config: SanitizedConf = serde_json::from_str(&conf_str).unwrap();

    println!("Config: {:#?}", config);
}
