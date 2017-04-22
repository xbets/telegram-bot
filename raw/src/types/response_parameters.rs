use serde::de::{Deserialize, Deserializer, Error};

use types::*;

/// All API responses are from this type. Mostly used internal.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Response<T> {
    /// Request was successful.
    Success {
        /// Response result.
        result: T,
    },
    /// Request was unsuccessful.
    Error {
        /// Human-readable description of the result.
        description: String,
        /// Contains information about why a request was unsuccessful.
        parameters: Option<ResponseParameters>,
    },
}

impl<T: Deserialize> Deserialize for Response<T> {
    fn deserialize<D>(deserializer: D) -> Result<Response<T>, D::Error> where D: Deserializer {
        let raw: RawResponse<T> = Deserialize::deserialize(deserializer)?;
        match (raw.ok, raw.description, raw.result) {
            (false, Some(description), None) => {
                Ok(Response::Error {
                    description: description,
                    parameters: raw.parameters,
                })
            },
            (true, None, Some(result)) => {
                Ok(Response::Success {
                    result: result,
                })
            }
            _ => Err(D::Error::custom("ambiguous response")),
        }
    }
}

/// Directly mapped telegram API response.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct RawResponse<T> {
    /// If ‘ok’ equals true, the request was successful.
    ok: bool,
    /// Human-readable description of the result.
    description: Option<String>,
    /// Result of the query.
    result: Option<T>,
    /// Information about why a request was unsuccessful.
    parameters: Option<ResponseParameters>,
}

/// Contains information about why a request was unsuccessful.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct ResponseParameters {
    /// The group has been migrated to a supergroup with the specified identifier.
    pub migrate_to_chat_id: Option<Integer>,
    /// In case of exceeding flood control, the number of seconds left to wait
    /// before the request can be repeated.
    pub retry_after: Option<Integer>,
}
