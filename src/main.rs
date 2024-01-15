use slint::{Weak, Timer, TimerMode};
use std::{process::{exit, Command}, time::Duration};

slint::include_modules!();

fn status_check() -> String {
    let output = Command::new("zsh")
        .arg("-c")
        .arg("warp-cli status")
        .output()
        .expect("Status cli error");

    if !output.status.success() {
        println!("Error in accessing warp cli");
        exit(1);
    }

    let output_str = String::from_utf8(output.stdout).unwrap();
    let mut result: &str = "";
    for i in output_str.split("\n") {
        if i.contains("Status") {
            result = i.split(":").collect::<Vec<_>>()[1].trim();
        }
    }
    String::from(result)
}

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

fn toggle_connection(action: bool) {
    let action_taken = if action { "connect" } else { "disconnect" };
    let output = Command::new("zsh")
        .arg("-c")
        .arg(format!("warp-cli {}", action_taken))
        .output()
        .unwrap();

    if !output.status.success() {
        println!("Error in accessing warp cli");
        exit(1);
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
