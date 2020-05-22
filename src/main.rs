extern crate clap;

use clap::{App, Arg};

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
    let matches = App::new("Launch Service")
        .version("1.0")
        .arg(
            Arg::with_name("query")
                .short("q")
                .long("query")
                .value_name("QUERY")
                .help("Query services")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("execute")
                .short("x")
                .long("execute")
                .value_name("ID")
                .help("Launch service with given id")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("alter_execute")
                .short("X")
                .long("alt-execute")
                .value_name("ID")
                .help("Reveal service with given id")
                .takes_value(true),
        )
        .get_matches();
    if let Some(query) = matches.value_of("query") {
        let output = std::env::var("OUTPUT").expect("Cannot get OUTPUT from env");
        let services = query::query(query.trim());
        let _ = std::fs::write("/Users/cheng/Desktop/output", output.as_str());
        std::fs::write(output, services).expect("Failed to write to OUTPUT");
    } else if let Some(id) = matches.value_of("execute") {
        execute::execute(id.trim(), false);
    } else if let Some(id) = matches.value_of("alter_execute") {
        execute::execute(id.trim(), true);
    }
}
