use thiserror::Error;
use barter_integration::socket::error::SocketError;

/// All errors generated in the barter::data module.
#[derive(Error, Copy, Clone, Debug)]
pub enum DataError {
    #[error("Invalid builder attributes provided")]
    BuilderAttributesInvalid,

    #[error("Failed to build struct due to incomplete attributes provided")]
    BuilderIncomplete,

    #[error("Socket: {0}")]
    Socket(#[from] SocketError),
}
