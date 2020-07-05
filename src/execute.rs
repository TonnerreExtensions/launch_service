use std::process::Command;

pub fn execute(id: &str, is_alter: bool) {
    let mut command = Command::new("open");
    let res = if is_alter {
        command.arg("-R").arg(id)
    } else {
        command.arg(id)
    }
    .spawn();
    if let Err(error) = res {
        println!("{}", error)
    }
}
