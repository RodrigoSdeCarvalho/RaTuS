#[derive(Debug)]
pub enum Error {
    TupleSpace(tuple_space::error::Error),
    OneShotRecv(tokio::sync::oneshot::error::RecvError),
    TomlDe(toml::de::Error),
    Io(std::io::Error),
}

impl From<tuple_space::error::Error> for Error {
    fn from(error: tuple_space::error::Error) -> Self {
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
