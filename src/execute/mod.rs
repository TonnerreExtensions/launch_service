use std::process::Command;

pub fn execute(id: &str) {
    Command::new("open")
        .arg(id)
        .spawn()
        .expect("Unable to open");
}