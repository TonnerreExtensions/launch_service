use std::io::stdout;

pub use query::QueryProcessor;

mod checker;
mod matcher;
mod query;
mod service;

pub fn query(req: &str) {
    QueryProcessor::new(stdout()).query(req);
}
