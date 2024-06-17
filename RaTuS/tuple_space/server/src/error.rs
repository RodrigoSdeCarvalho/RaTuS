#[derive(Debug)]
pub enum Error {
    TupleSpace(ts_core::error::Error),
    OneShotRecv(tokio::sync::oneshot::error::RecvError),
    TomlDe(toml::de::Error),
    Io(std::io::Error),
}

impl From<ts_core::error::Error> for Error {
    fn from(error: ts_core::error::Error) -> Self {
        Error::TupleSpace(error)
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for Error {
    fn from(error: tokio::sync::oneshot::error::RecvError) -> Self {
        Error::OneShotRecv(error)
    }
}

impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self {
        Error::TomlDe(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}
