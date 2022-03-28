mod macros;
mod register_auto_launch;
mod server;
mod system_tray;

use std::{env, fs, time::Duration};

use chrono::{Local, NaiveDateTime};
use deadliner_gui::{new_path, update_wallpaper, SanitizedConf};
pub use macros::*;
pub use register_auto_launch::*;
pub use server::*;
pub use system_tray::*;
use tokio_cron_scheduler::{Job, JobScheduler};

pub fn start_schedular() {
    let conf_str =
        fs::read_to_string(new_path("config.json")).expect("Can't read Config JSON file!");

    let conf: SanitizedConf = serde_json::from_str(&conf_str).unwrap();

    let args: Vec<String> = env::args().collect();
    let skip_update_on_startup = args.get(1) == Some(&"skip-update-on-launch".to_string());

    if !skip_update_on_startup {
        // Run on OS launch
        update_wallpaper(&conf).unwrap();
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
                let today = Local::now().naive_local();
                let deadline =
                    NaiveDateTime::parse_from_str(&conf.deadline_str, "%Y-%m-%d %I:%M %p").unwrap();
                let diff = deadline.signed_duration_since(today);

                let minutes = diff.num_minutes();

                if minutes < 60 {
                    update_wallpaper(&conf).unwrap();
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
        update_wallpaper(&conf).unwrap();
    })
    .unwrap();

    job
}
