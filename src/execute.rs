use std::process::Command;

pub fn execute(id: &str, is_alter: bool) {
    let mut command = Command::new("open");
    if is_alter {
        command.arg("-R").arg(id)
    } else {
        command.arg(id)
    }.spawn().expect("Unable to execute");
}

