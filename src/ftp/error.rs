use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

/// Represents various errors that can occur during FTP operations.
#[derive(Debug, From)]
pub enum Error {
    /// Error occurred while reading data.
    ReadError,

    /// Error occurred while attempting to reconnect.
    ReconnectError,

    /// Error occurred while establishing a connection.
    ConnectionError,

    /// Error occurred while resolving the domain.
    DomainResolutionError,

    /// Error occurred while writing a command.
    CommandWriteError,

    /// Error occurred while flushing a command.
    CommandFlushError,

    /// Error occurred due to invalid parsed data.
    InvalidParsedData,

    /// Error occurred due to invalid parsed IP address.
    InvalidParsedIp,

    /// Error occurred due to invalid parsed port.
    InvalidParsedPort,

    /// No response was received.
    NoResponseReceived,
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
