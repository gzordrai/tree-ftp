use crate::ftp::error::Result;

pub trait Reconnectable {
    fn reconnect(&mut self) -> Result<()>;
}