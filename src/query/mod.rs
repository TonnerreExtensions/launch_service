pub use query::QueryProcessor;

mod checker;
mod matcher;
mod query;
mod response;
mod service;

pub fn query(req: &str) -> Vec<u8> {
    QueryProcessor::new().query(req)
}
