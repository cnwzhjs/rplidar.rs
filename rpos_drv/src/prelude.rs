use std::io;

pub use super::errors::*;

/// A message send via channels
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    /// The command
    pub cmd: u8,

    /// Payload data
    pub data: Vec<u8>
}

impl Message {
    pub fn new(cmd: u8) -> Message {
        return Message::with_data(cmd, &[]);
    }

    pub fn with_data(cmd: u8, data: &[u8]) -> Message {
        let mut msg = Message {
            cmd: cmd,
            data: Vec::with_capacity(data.len())
        };

        msg.data.extend_from_slice(&data);

        return msg;
    }
}

/// Protocol decoder
pub trait ProtocolDecoder {
    /// Decode bytes and return consumed bytes and message
    fn decode(&mut self, buf: &[u8]) -> Result<(usize, Option<Message>)>;

    /// Reset the decoder status
    fn reset_decoder(&mut self);
}

/// Protocol encoder
pub trait ProtocolEncoder {
    /// Encode message into byte array
    fn encode(&mut self, msg: &Message, bytes: &mut [u8]) -> Result<usize>;

    /// Estimate encoded message size (must be greater than or equal to the actual encoded size)
    fn estimate_encoded_size(&mut self, msg: &Message) -> Result<usize>;

    /// Write message to `std::io::Write` targets
    fn write_to(&mut self, msg: &Message, dest: &mut impl io::Write) -> Result<usize>;

    /// Reset encoder
    fn reset_encoder(&mut self);
}
