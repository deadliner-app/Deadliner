mod macros;
mod notify;
mod server;
mod startup_launch;
mod system_tray;

use std::{
    env, fs,
    sync::{Arc, Mutex},
    time::Instant,
};

use chrono::{Local, NaiveDateTime};
use deadliner_gui::{generate_deadline_over_wallpaper, new_path, update_wallpaper, SanitizedConf};
pub use macros::*;
pub use notify::*;
pub use server::*;
pub use startup_launch::*;
pub use system_tray::*;
use tokio_cron_scheduler::{Job, JobScheduler};

pub fn start_schedular(exit: Arc<Mutex<bool>>) {
    let conf_str =
        fs::read_to_string(new_path("config.json")).expect("Can't read Config JSON file!");

    let conf: SanitizedConf = serde_json::from_str(&conf_str).unwrap();

    let args: Vec<String> = env::args().collect();
    let skip_update_on_startup = args.get(1) == Some(&"skip-update-on-launch".to_string());
    let mut schedule = true;

    if !skip_update_on_startup {
        // Run on OS launch

        let minutes = get_minutes_left(&conf);
        if minutes <= 0 {
            set_deadline_is_over(&conf);

            let mut exit = exit.lock().unwrap();
            *exit = true;
            schedule = false;
        } else if minutes < 60 {
            update_wallpaper(&conf, false).unwrap();
        }
    }

    if !schedule {
        return;
    }

    let mut sched = JobScheduler::new();

    if conf.show_hours {
        // Run every minute 0 (aka: every begining of a local hour)
        sched
            .add(instantiate_job("* 0 * * * * *", conf.clone()))
            .unwrap();
    } else if conf.show_days {
        // Run every midnight
        sched
            .add(instantiate_job("* 0 0 * * * * *", conf.clone()))
            .unwrap();
    } else if conf.show_weeks {
        // Run every week
        // First day in the week = Sunday.
        // TODO: ask for the weekend of a user.
        sched
            .add(instantiate_job("* 0 0 * * 7 *", conf.clone()))
            .unwrap();
    } else if conf.show_months {
        // Run every month
        sched
            .add(instantiate_job("* 0 0 1 * * *", conf.clone()))
            .unwrap();
    }

    // Setup another schedule that run every minute to check if we're near the deadline
    // by less than 60 minutes
    sched
        .add(
            Job::new("0 * * * * * *", move |_uuid, _l| {
                let minutes = get_minutes_left(&conf);

                // Check every minute if the deadline is over.
                // If so, exit and remove schedular from auto-startup
                if minutes <= 0 {
                    set_deadline_is_over(&conf);

                    let mut exit = exit.lock().unwrap();
                    *exit = true;
                } else if minutes < 60 {
                    update_wallpaper(&conf, false).unwrap();
                }
            })
            .unwrap(),
        )
        .unwrap();

    sched.start();
}

fn instantiate_job<'a>(cron: &str, conf: SanitizedConf) -> Job {
    let job = Job::new(cron, move |_uuid, _l| {
        // Setup minutes schedular if deadline is under 60 minutes
        update_wallpaper(&conf, false).unwrap();
    })
    .unwrap();

    job
}

fn set_deadline_is_over(conf: &SanitizedConf) {
    unregister_auto_launch();
    fs::remove_file(new_path("config.json")).unwrap();

    let file_path = generate_deadline_over_wallpaper("Deadline is Over", &conf);

    match file_path {
        Ok(file_path) => {
            // Sets the wallpaper for the current desktop from a URL.
            wallpaper::set_mode(conf.bg_mode.into()).unwrap();
            wallpaper::set_from_path(&file_path).unwrap();
        }
        _ => {}
    }

    notify_deadline_over();
}

fn get_minutes_left(conf: &SanitizedConf) -> i64 {
    let today = Local::now().naive_local();
    let deadline = NaiveDateTime::parse_from_str(&conf.deadline_str, "%Y-%m-%d %I:%M %p").unwrap();
    let diff = deadline.signed_duration_since(today);

    let minutes = diff.num_minutes();

    minutes
}
