use crate::ftp::error::Result;
use crate::ftp::stream::Stream;
use log::info;
use std::net::{SocketAddr, TcpStream};

pub struct DataStream {
    addr: SocketAddr,
    stream: TcpStream,
    reconnected: bool,
}

impl DataStream {
    pub fn new(addr: SocketAddr) -> Result<Self> {
        let stream: TcpStream = TcpStream::connect(addr)?;

        info!("Connected to the server");

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
        return self.reconnected;
    }
}
