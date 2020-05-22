use std::iter::FromIterator;

use serde::Serialize;

use crate::utils;

static IDENTIFIER: &str = "IDENTIFIER";

#[derive(Serialize)]
pub struct Response<S: Serialize> {
    services: Vec<S>,
    provider: String,
}

impl<S: Serialize> FromIterator<S> for Response<S> {
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        let services = iter.into_iter().collect::<Vec<_>>();
        let provider = std::env::var(IDENTIFIER).expect("Unable to get IDENTIFIER from env");
        Response { services, provider }
    }
}

impl<S: Serialize> Response<S> {
    pub fn serialize_to_json(self) -> Vec<u8> {
        utils::serde::serialize_to_bytes(&self).unwrap_or_default()
    }
}
