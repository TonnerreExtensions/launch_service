use std::iter::FromIterator;

use serde::Serialize;

static IDENTIFIER: &str = "IDENTIFIER";

#[derive(Serialize)]
pub struct Response<S: Serialize> {
    services: Vec<S>,
    identifier: String,
}

impl<S: Serialize> FromIterator<S> for Response<S> {
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        let services = iter.into_iter().collect::<Vec<_>>();
        let identifier = std::env::var(IDENTIFIER).expect("Unable to get IDENTIFIER from env");
        Response {
            services,
            identifier,
        }
    }
}

impl<S: Serialize> Response<S> {
    pub fn serialize_to_json(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap_or(vec![])
    }
}
