#[derive(Debug)]
pub enum Error {
    SerdeJson(serde_json::Error),
    Reqwest(reqwest::Error),
    ServerError,
    UrlParser(url::ParseError),
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Error::SerdeJson(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        Error::Reqwest(error)
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Error {
        Error::UrlParser(error)
    }
}
