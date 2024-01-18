use slint::{Timer, TimerMode, Weak};
use std::time::Duration;

use warp_gui_app::{status_check, toggle_connection};

slint::include_modules!();

fn __init__(ui: Weak<AppWindow>) {
    let ui = ui.unwrap();
    let result = status_check();
    if result == "Connected" {
        ui.set_connectionState(1);
    } else if result == "Disconnected. Reason" {
        ui.set_connectionState(0);
    } else {
        ui.set_connectionState(2);
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let timer = Timer::default();
    let thread_ui = ui.as_weak();
    timer.start(TimerMode::Repeated, Duration::from_secs(1), move || {
        let ui_copy = thread_ui.clone();
        __init__(ui_copy);
    });

    let ui_toggle = ui.as_weak();
    ui.on_toggle(move || {
        let ui = ui_toggle.unwrap();
        let state = ui.get_connectionState();
        if state == 0 {
            toggle_connection(true);
            ui.set_connectionState(2);
            println!("{}", status_check());
        } else {
            toggle_connection(false);
            ui.set_connectionState(0);
            println!("{}", status_check());
        }
    });

    ui.run()
}
