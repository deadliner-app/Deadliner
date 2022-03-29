use notify_rust::Notification;

pub fn notify_deadline_over() {
    Notification::new()
        .summary("⌚ Times up!")
        .body("Your deadline is over, hopefully you've achieved whatever you were tryna do 👀")
        .auto_icon()
        .show()
        .unwrap();
}
