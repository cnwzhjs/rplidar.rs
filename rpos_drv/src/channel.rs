use super::ring_byte_buffer::RingByteBuffer;
use super::prelude::*;
use std::io;
use std::time::{ Instant, Duration };
use failure::{Error, Fail};

const DEFAULT_CHANNEL_READ_BUFFER_SIZE: usize = 1024;

/// Channel encode and decode message with protocol, and send and receive bytes via stream
/// 
/// # Examples
/// ```ignore
/// let mut channel = Channel::new(
///     RplidarProtocol::new(),
///     serial_port
/// );
/// 
/// channel.write(&Message::new(1)).unwrap();
/// ```
#[derive(Debug)]
pub struct Channel<P, T: ?Sized> {
    protocol: P,
    stream: Box<T>,
    read_buffer: RingByteBuffer,
}

impl<P, T: ?Sized> Channel<P, T>
where
    P: ProtocolDecoder + ProtocolEncoder,
    T: io::Read + io::Write
{
    /// Create a new `Channel` to read and write messages
    /// 
    /// # Example
    /// ```ignore
    /// let channel = Channel::new(
    ///     RplidarProtocol::new(),
    ///     serial_port
    /// );
    /// ```
    pub fn new(protocol: P, stream: Box<T>) -> Channel<P, T> {
        Channel::with_read_buffer_size(protocol, stream, DEFAULT_CHANNEL_READ_BUFFER_SIZE)
    }

    /// Create a new `Channel` with non-default ring buffer capacity
    /// 
    /// # Example
    /// ```ignore
    /// let channel = Channel::with_read_buffer_size(
    ///     RplidarProtocol::new(),
    ///     serial_port,
    ///     100000 as usize
    /// );
    /// ```
    pub fn with_read_buffer_size(
        protocol: P,
        stream: Box<T>,
        read_buffer_size: usize,
    ) -> Channel<P, T> {
        let mut chn = Channel {
            protocol: protocol,
            stream: stream,
            read_buffer: RingByteBuffer::with_capacity(read_buffer_size),
        };

        chn.reset();

        return chn;
    }

    /// Reset the channel status
    /// This function is usually used to reset protocol encoder and decoder when meet communication error
    /// 
    /// # Example
    /// ```ignore
    /// match channel.invoke(&Message::new(1), Duration::from_secs(1)) {
    ///     Ok(_) => {},
    ///     Err(_) => { channel.reset(); }
    /// }
    /// ```
    pub fn reset(&mut self) {
        self.protocol.reset_encoder();
        self.protocol.reset_decoder();
    }

    /// Read message from channel
    /// 
    /// # Example
    /// ```ignore
    /// if let Some(msg) = channel.read().unwrap() {
    ///     println!("{:?}", msg);
    /// }
    /// ```
    pub fn read(&mut self) -> Result<Option<Message>> {
        loop {
            self.read_buffer.read_from(&mut self.stream)?;

            let (decoded, msg) = self
                .protocol
                .decode(self.read_buffer.current_read_slice())?;
            self.read_buffer.skip_bytes(decoded);

            if decoded == 0 {
                return Ok(Option::None);
            }

            if let Some(_) = msg {
                return Ok(msg);
            }
        }
    }

    /// Read message until timeout
    /// 
    /// # Example
    /// ```ignore
    /// channel.read_until(Duration::from_secs(1));
    /// ```
    pub fn read_until(&mut self, timeout: Duration) -> Result<Option<Message>> {
        let start = Instant::now();

        while Instant::now() - start < timeout {
            if let Some(msg) = self.read()? {
                return Ok(Some(msg));
            }
        }

        return Err(RposError::OperationTimeout.into());
    }

    /// Write message to channel
    /// 
    /// # Example
    /// ```ignore
    /// channel.write(&Message::new(1)).unwrap();
    /// ```
    pub fn write(&mut self, msg:&Message) -> Result<usize> {
        let written = self.protocol.write_to(msg, &mut self.stream)?;
        self.stream.flush()?;
        return Ok(written);
    }

    /// Send a request to channel and wait for response
    /// 
    /// # Example
    /// ```ignore
    /// let resp = channel.invoke(&Message::new(1), Duration::from_secs(1));
    /// ```
    pub fn invoke(&mut self, request:&Message, timeout: Duration) -> Result<Option<Message>> {
        self.write(request)?;
        return self.read_until(timeout);
    }
    
}
