pub use query::QueryProcessor;

mod query;
mod checker;
mod matcher;
mod service;

pub fn query(req: &str) -> Vec<u8> {
    QueryProcessor::new().query(req)
}