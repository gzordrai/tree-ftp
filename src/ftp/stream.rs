use crate::ftp::error::{Error, Result};
use log::{debug, error, info};
use std::io::{BufRead, BufReader};
use std::{
    net::{SocketAddr, TcpStream},
    thread::sleep,
    time::{Duration, Instant},
};

/// Represents a response from the FTP server.
pub type Response = (u16, String);

/// Represents multiple responses from the FTP server.
pub type Responses = Vec<Response>;

/// A trait for managing TCP streams in FTP operations.
pub trait Stream {
    /// Returns a reference to the TCP stream.
    fn get_stream(&self) -> &TcpStream;

    /// Returns the address of the data stream.
    fn get_addr(&self) -> SocketAddr;

    /// Sets the TCP stream.
    ///
    /// # Arguments
    ///
    /// * `stream` - The new TCP stream.
    fn set_stream(&mut self, stream: TcpStream);

    /// Sets the reconnected status.
    ///
    /// # Arguments
    ///
    /// * `reconnected` - The new reconnected status.
    fn set_reconnected(&mut self, reconnected: bool);

    /// Returns whether the stream has been reconnected.
    fn is_reconnected(&mut self) -> bool;

    /// Attempts to reconnect the TCP stream.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the reconnection was successful.
    fn reconnect(&mut self) -> Result<bool> {
        let addr: SocketAddr = self.get_addr();
        let start_time: Instant = Instant::now();
        let timeout: Duration = Duration::from_secs(300); // 5 minutes
        let retry_interval: Duration = Duration::from_secs(5); // Retry every 5 seconds

        sleep(Duration::from_secs(5)); // For localhost docker ftp

        while start_time.elapsed() < timeout {
            match TcpStream::connect(addr) {
                Ok(new_stream) => {
                    self.set_stream(new_stream);

                    info!("Reconnected to the server at {}", addr);

                    self.set_reconnected(true);
                    self.read_responses()?;

                    return Ok(true);
                }
                Err(_) => {
                    error!(
                        "Failed to reconnect to the server at {}. Retrying in 5 seconds...",
                        addr
                    );

                    sleep(retry_interval);
                }
            }
        }

        error!(
            "Failed to reconnect to the server at {} after 5 minutes",
            addr
        );

        Err(Error::ReconnectError)
    }

    /// Reads responses from the TCP stream.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of responses from the FTP server, or an `Error` if reading fails.
    fn read_responses(&mut self) -> Result<Vec<Response>> {
        let stream: &TcpStream = self.get_stream();
        let mut reader: BufReader<&TcpStream> = BufReader::new(stream);
        let mut responses: Vec<Response> = Vec::new();

        loop {
            let mut line: String = String::new();
            let bytes_read: usize = match reader.read_line(&mut line) {
                Ok(bytes_read) => bytes_read,
                Err(e) => {
                    if let Some(10053) | Some(libc::ECONNABORTED) = e.raw_os_error() {
                        error!("Connection was aborted by the software in your host machine. Attempting to reconnect...");

                        self.reconnect()?;

                        return Ok(Vec::new());
                    }

                    return Err(Error::ReadError);
                }
            };

            if bytes_read == 0 {
                break;
            }

            debug!("Read line: {}", line.trim_end());

            let code = if line.len() >= 3 {
                line[0..3].parse::<u16>().unwrap_or(0)
            } else {
                0
            };

            responses.push((code, line.trim_end().to_string()));

            if line.len() >= 4 && &line[3..4] == " " {
                break;
            }
        }

        debug!("Full response: {:?}", responses);

        Ok(responses)
    }
}
