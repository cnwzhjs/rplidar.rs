use std::error;
use std::io::Error as IoError;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        /// The execution of operation failed
        OperationFail(description: String) {
            description(description)
        }

        /// The execution of operation is timed out
        OperationTimeout {}

        /// The device doesn't support this operation
        OperationNotSupport {}

        /// The decoding data is invalid according to current protocol
        ProtocolError(description: String) {
            description(description)
        }

        /// The buffer is too small for message encoding
        BufferTooSmall {}

        Io(err: IoError) {
            from()
            cause(err)
            description(err.description())
            display("{:?}", err)
        }

        Other(err: Box<dyn error::Error + Sync + Send>) {
            from()
            cause(err.as_ref())
            description(err.description())
            display("{:?}", err)
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;