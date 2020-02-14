extern crate clap;
#[macro_use]
extern crate lazy_static;

use std::io::{stdout, Write};

use clap::{App, Arg};

mod configurator;
mod query;
mod execute;
mod utils;

fn main() {
    let matches = App::new("Launch Service")
        .version("1.0")
        .arg(Arg::with_name("query")
            .short("q")
            .long("query")
            .value_name("QUERY")
            .help("Query services")
            .takes_value(true)
        ).arg(Arg::with_name("execute")
        .short("x")
        .long("execute")
        .value_name("ID")
        .help("Launch service with given id")
        .takes_value(true)
    ).get_matches();
    if let Some(query) = matches.value_of("query") {
        let services = query::query(query);
        stdout().lock().write(&services).expect("Unable to write to stdout");
    } else if let Some(id) = matches.value_of("execute") {
        execute::execute(id);
    }
}
