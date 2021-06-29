mod api_datamodel;
pub mod datamodel;
mod mealdb;
pub mod traits;

pub use crate::mealdb::V1;

#[derive(Debug)]
pub enum Error {
    DeserializationFailure(serde_json::Error),
    RequestFalure(reqwest::Error),
    InvalidAPIResponse,
    Unknown(),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::RequestFalure(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::DeserializationFailure(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
