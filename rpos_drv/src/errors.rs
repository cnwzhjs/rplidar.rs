use std::error;
pub use failure::{ Fail, Error };

#[derive(Fail, Debug)]
pub enum RposError {
    /// The execution of operation failed
    #[fail(display="operation failed: {}", description)]
    OperationFail {
        description: String
    },

    /// The execution of operation is timed out
    #[fail(display="operation timeout")]
    OperationTimeout,

    /// The device doesn't support this operation
    #[fail(display="operation not support")]
    OperationNotSupport,

    /// The decoding data is invalid according to current protocol
    #[fail(display="protocol error: {}", description)]
    ProtocolError {
        description: String
    },

    /// The buffer is too small for message encoding
    #[fail(display="buffer is too small for message encoding")]
    BufferTooSmall
}

pub type Result<T> = std::result::Result<T, Error>;
