use lazy_static::lazy_static;

mod configurator;
mod execute;
mod query;
mod utils;

lazy_static! {
    pub static ref CONFIG: configurator::Configs = configurator::Configs::from(
        std::env::var("SETTINGS").expect("Failed to get SETTINGS from environment")
    )
    .expect("settings is invalid");
}

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let action = args.next().expect("Action flag is missing");
    let content = args.next().expect("Content is missing");
    match action.trim() {
        "-q" | "--query" => query::query(&content),
        "-x" | "--execute" => execute::execute(&content, false),
        "-X" | "--alter-execute" => execute::execute(&content, true),
        _ => panic!("Unexpected flag"),
    };
}
