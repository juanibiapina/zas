use std::io;

#[derive(Debug)]
pub enum Error {
    InvalidUserHome,
    InvalidPort(String),
    AppNotConfigured,

    IoError(io::Error),
    XdgError(xdg::BaseDirectoriesError),
    ConfigDeserializationError(toml::de::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<xdg::BaseDirectoriesError> for Error {
    fn from(err: xdg::BaseDirectoriesError) -> Error {
        Error::XdgError(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Error {
        Error::ConfigDeserializationError(err)
    }
}
