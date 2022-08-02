pub enum Error {
    FailedToReadConfigurationFile(Box<dyn std::error::Error>),
    FailedToParseConfigurationFile(Box<dyn std::error::Error>),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::FailedToReadConfigurationFile(Box::new(err))
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        return Self::FailedToParseConfigurationFile(Box::new(err));
    }
}
