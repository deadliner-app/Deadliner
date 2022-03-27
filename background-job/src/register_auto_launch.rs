use auto_launch::AutoLaunch;
use std::env;

#[cfg(target_os = "windows")]
pub fn register_auto_launch() {
    let app_name = "deadliner-schedular";
    let app_path = env::current_exe().unwrap();
    let app_path = app_path.to_str().unwrap();

    let auto = AutoLaunch::new(app_name, app_path);

    let is_enabled = auto.is_enabled().unwrap();
    if !is_enabled {
        auto.enable().unwrap();
    }
}

#[cfg(target_os = "macos")]
pub fn register_auto_launch() {
    let app_name = "deadliner-schedular";
    let app_path = env::current_exe().unwrap();
    let app_path = app_path.to_str().unwrap();
    let auto = AutoLaunch::new(app_name, app_path, false, false);

    let is_enabled = auto.is_enabled().unwrap();
    if !is_enabled {
        auto.enable().unwrap();
    }
}

#[cfg(target_os = "linux")]
pub fn register_auto_launch() {
    let app_name = "deadliner-schedular";
    let app_path = env::current_exe().unwrap();
    let app_path = app_path.to_str().unwrap();

    let auto = AutoLaunch::new(app_name, app_path, false);

    let is_enabled = auto.is_enabled().unwrap();
    if !is_enabled {
        auto.enable().unwrap();
    }
}
