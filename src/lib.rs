use std::process::{exit, Command};

slint::include_modules!();

pub fn status_check() -> String {
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

pub fn toggle_connection(action: bool) {
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
