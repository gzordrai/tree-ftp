use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    ReadError,
    ReconnectError,
    ConnectionError,
    DomainResolutionError,
    CommandWriteError,
    CommandFlushError,
    InvalidParsedData,
    InvalidParsedIp,
    InvalidParsedPort,
    NoResponseReceived
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}