use crate::ftp::error::{Error, Result};
use crate::ftp::stream::Stream;
use log::debug;
use std::net::{SocketAddr, TcpStream};

/// Represents a data stream for FTP communication.
pub struct DataStream {
    addr: SocketAddr,
    stream: TcpStream,
    reconnected: bool,
}

impl DataStream {
    /// Creates a new `DataStream`.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address of the data stream.
    /// * `stream` - The TCP stream used for communication.
    ///
    /// # Returns
    ///
    /// A new `DataStream` instance.
    pub fn new(addr: SocketAddr) -> Result<Self> {
        let stream: TcpStream = TcpStream::connect(addr).map_err(|_| Error::ConnectionError)?;

        debug!("Connected to the data server");

        Ok(DataStream {
            addr,
            stream,
            reconnected: false,
        })
    }
}

impl Stream for DataStream {
    fn get_stream(&self) -> &TcpStream {
        &self.stream
    }

    fn get_addr(&self) -> SocketAddr {
        self.addr
    }

    fn set_stream(&mut self, stream: TcpStream) {
        self.stream = stream;
    }

    fn set_reconnected(&mut self, reconnected: bool) {
        self.reconnected = reconnected;
    }

    fn is_reconnected(&mut self) -> bool {
        self.reconnected
    }
}
