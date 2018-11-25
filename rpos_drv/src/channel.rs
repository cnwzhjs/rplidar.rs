use super::ring_byte_buffer::RingByteBuffer;
use super::prelude::{Message, ProtocolDecoder, ProtocolEncoder, Result, Error, ErrorKind};
use std::io;
use std::time::{ Instant, Duration };

const DEFAULT_CHANNEL_READ_BUFFER_SIZE: usize = 1024;

/// Channel encode and decode message with protocol, and send and receive bytes via stream
/// 
/// # Examples
/// ```rust
/// # use rpos_drv::{ ProtocolDecoder, ProtocolEncoder, Channel, Message, Result, RingByteBuffer };
/// # use std::io::{ Read, Write };
/// # use std::fs::File;
/// # let mut serial_port = Box::new(RingByteBuffer::with_capacity(100));
/// # struct RplidarProtocol {}
/// # impl RplidarProtocol {
/// #   pub fn new() -> RplidarProtocol {
/// #     RplidarProtocol {}
/// #   }
/// # }
/// # impl ProtocolDecoder for RplidarProtocol {
/// #   fn decode(&mut self, buf: &[u8]) -> Result<(usize, Option<Message>)> {
/// #       Ok((0 as usize, None))
/// #   }
/// #   fn reset_decoder(&mut self) {}
/// # }
/// # impl ProtocolEncoder for RplidarProtocol {
/// #   fn encode(&mut self, msg: &Message, bytes: &mut[u8]) -> Result<usize> {
/// #     Ok(0 as usize)
/// #   }
/// #   fn estimate_encoded_size(&mut self, msg: &Message) -> Result<usize> {
/// #     Ok(0 as usize)
/// #   }
/// #   fn write_to(&mut self, msg: &Message, dest: &mut impl Write) -> Result<usize> {
/// #     Ok(0 as usize)
/// #   }
/// #   fn reset_encoder(&mut self) {}
/// # }
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
    /// ```rust
    /// # use rpos_drv::{ ProtocolDecoder, ProtocolEncoder, Channel, Message, Result, RingByteBuffer };
    /// # use std::io::{ Read, Write };
    /// # use std::fs::File;
    /// # let mut serial_port = Box::new(RingByteBuffer::with_capacity(100));
    /// # struct RplidarProtocol {}
    /// # impl RplidarProtocol {
    /// #   pub fn new() -> RplidarProtocol {
    /// #     RplidarProtocol {}
    /// #   }
    /// # }
    /// # impl ProtocolDecoder for RplidarProtocol {
    /// #   fn decode(&mut self, buf: &[u8]) -> Result<(usize, Option<Message>)> {
    /// #       Ok((0 as usize, None))
    /// #   }
    /// #   fn reset_decoder(&mut self) {}
    /// # }
    /// # impl ProtocolEncoder for RplidarProtocol {
    /// #   fn encode(&mut self, msg: &Message, bytes: &mut[u8]) -> Result<usize> {
    /// #     Ok(0 as usize)
    /// #   }
    /// #   fn estimate_encoded_size(&mut self, msg: &Message) -> Result<usize> {
    /// #     Ok(0 as usize)
    /// #   }
    /// #   fn write_to(&mut self, msg: &Message, dest: &mut impl Write) -> Result<usize> {
    /// #     Ok(0 as usize)
    /// #   }
    /// #   fn reset_encoder(&mut self) {}
    /// # }
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
    /// ```rust
    /// # use rpos_drv::{ ProtocolDecoder, ProtocolEncoder, Channel, Message, Result, RingByteBuffer };
    /// # use std::io::{ Read, Write };
    /// # use std::fs::File;
    /// # let mut serial_port = Box::new(RingByteBuffer::with_capacity(100));
    /// # struct RplidarProtocol {}
    /// # impl RplidarProtocol {
    /// #   pub fn new() -> RplidarProtocol {
    /// #     RplidarProtocol {}
    /// #   }
    /// # }
    /// # impl ProtocolDecoder for RplidarProtocol {
    /// #   fn decode(&mut self, buf: &[u8]) -> Result<(usize, Option<Message>)> {
    /// #       Ok((0 as usize, None))
    /// #   }
    /// #   fn reset_decoder(&mut self) {}
    /// # }
    /// # impl ProtocolEncoder for RplidarProtocol {
    /// #   fn encode(&mut self, msg: &Message, bytes: &mut[u8]) -> Result<usize> {
    /// #     Ok(0 as usize)
    /// #   }
    /// #   fn estimate_encoded_size(&mut self, msg: &Message) -> Result<usize> {
    /// #     Ok(0 as usize)
    /// #   }
    /// #   fn write_to(&mut self, msg: &Message, dest: &mut impl Write) -> Result<usize> {
    /// #     Ok(0 as usize)
    /// #   }
    /// #   fn reset_encoder(&mut self) {}
    /// # }
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
    /// ```rust
    /// # use rpos_drv::{ ProtocolDecoder, ProtocolEncoder, Channel, Message, Result, RingByteBuffer };
    /// # use std::io::{ Read, Write };
    /// # use std::fs::File;
    /// # use std::time::Duration;
    /// # let mut serial_port = Box::new(RingByteBuffer::with_capacity(100));
    /// # struct RplidarProtocol {}
    /// # impl RplidarProtocol {
    /// #   pub fn new() -> RplidarProtocol {
    /// #     RplidarProtocol {}
    /// #   }
    /// # }
    /// # impl ProtocolDecoder for RplidarProtocol {
    /// #   fn decode(&mut self, buf: &[u8]) -> Result<(usize, Option<Message>)> {
    /// #       Ok((0 as usize, None))
    /// #   }
    /// #   fn reset_decoder(&mut self) {}
    /// # }
    /// # impl ProtocolEncoder for RplidarProtocol {
    /// #   fn encode(&mut self, msg: &Message, bytes: &mut[u8]) -> Result<usize> {
    /// #     Ok(0 as usize)
    /// #   }
    /// #   fn estimate_encoded_size(&mut self, msg: &Message) -> Result<usize> {
    /// #     Ok(0 as usize)
    /// #   }
    /// #   fn write_to(&mut self, msg: &Message, dest: &mut impl Write) -> Result<usize> {
    /// #     Ok(0 as usize)
    /// #   }
    /// #   fn reset_encoder(&mut self) {}
    /// # }
    /// # let mut channel = Channel::new(RplidarProtocol::new(), serial_port);
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
    /// ```rust
    /// # use rpos_drv::{ ProtocolDecoder, ProtocolEncoder, Channel, Message, Result, RingByteBuffer };
    /// # use std::io::{ Read, Write };
    /// # use std::fs::File;
    /// # let mut serial_port = Box::new(RingByteBuffer::with_capacity(100));
    /// # struct RplidarProtocol {}
    /// # impl RplidarProtocol {
    /// #   pub fn new() -> RplidarProtocol {
    /// #     RplidarProtocol {}
    /// #   }
    /// # }
    /// # impl ProtocolDecoder for RplidarProtocol {
    /// #   fn decode(&mut self, buf: &[u8]) -> Result<(usize, Option<Message>)> {
    /// #       Ok((0 as usize, None))
    /// #   }
    /// #   fn reset_decoder(&mut self) {}
    /// # }
    /// # impl ProtocolEncoder for RplidarProtocol {
    /// #   fn encode(&mut self, msg: &Message, bytes: &mut[u8]) -> Result<usize> {
    /// #     Ok(0 as usize)
    /// #   }
    /// #   fn estimate_encoded_size(&mut self, msg: &Message) -> Result<usize> {
    /// #     Ok(0 as usize)
    /// #   }
    /// #   fn write_to(&mut self, msg: &Message, dest: &mut impl Write) -> Result<usize> {
    /// #     Ok(0 as usize)
    /// #   }
    /// #   fn reset_encoder(&mut self) {}
    /// # }
    /// # let mut channel = Channel::new(RplidarProtocol::new(), serial_port);
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
    /// ```rust
    /// # use std::time::Duration;
    /// # use rpos_drv::{ ProtocolDecoder, ProtocolEncoder, Channel, Message, Result, RingByteBuffer };
    /// # use std::io::{ Read, Write };
    /// # use std::fs::File;
    /// # let mut serial_port = Box::new(RingByteBuffer::with_capacity(100));
    /// # struct RplidarProtocol {}
    /// # impl RplidarProtocol {
    /// #   pub fn new() -> RplidarProtocol {
    /// #     RplidarProtocol {}
    /// #   }
    /// # }
    /// # impl ProtocolDecoder for RplidarProtocol {
    /// #   fn decode(&mut self, buf: &[u8]) -> Result<(usize, Option<Message>)> {
    /// #       Ok((0 as usize, None))
    /// #   }
    /// #   fn reset_decoder(&mut self) {}
    /// # }
    /// # impl ProtocolEncoder for RplidarProtocol {
    /// #   fn encode(&mut self, msg: &Message, bytes: &mut[u8]) -> Result<usize> {
    /// #     Ok(0 as usize)
    /// #   }
    /// #   fn estimate_encoded_size(&mut self, msg: &Message) -> Result<usize> {
    /// #     Ok(0 as usize)
    /// #   }
    /// #   fn write_to(&mut self, msg: &Message, dest: &mut impl Write) -> Result<usize> {
    /// #     Ok(0 as usize)
    /// #   }
    /// #   fn reset_encoder(&mut self) {}
    /// # }
    /// # let mut channel = Channel::new(RplidarProtocol::new(), serial_port);
    /// channel.read_until(Duration::from_secs(1));
    /// ```
    pub fn read_until(&mut self, timeout: Duration) -> Result<Option<Message>> {
        let start = Instant::now();

        while Instant::now() - start < timeout {
            if let Some(msg) = self.read()? {
                return Ok(Some(msg));
            }
        }

        return Err(Error::new(ErrorKind::OperationTimeout, "operation timeout"));
    }

    /// Write message to channel
    /// 
    /// # Example
    /// ```rust
    /// # use rpos_drv::{ ProtocolDecoder, ProtocolEncoder, Channel, Message, Result, RingByteBuffer };
    /// # use std::io::{ Read, Write };
    /// # use std::fs::File;
    /// # let mut serial_port = Box::new(RingByteBuffer::with_capacity(100));
    /// # struct RplidarProtocol {}
    /// # impl RplidarProtocol {
    /// #   pub fn new() -> RplidarProtocol {
    /// #     RplidarProtocol {}
    /// #   }
    /// # }
    /// # impl ProtocolDecoder for RplidarProtocol {
    /// #   fn decode(&mut self, buf: &[u8]) -> Result<(usize, Option<Message>)> {
    /// #       Ok((0 as usize, None))
    /// #   }
    /// #   fn reset_decoder(&mut self) {}
    /// # }
    /// # impl ProtocolEncoder for RplidarProtocol {
    /// #   fn encode(&mut self, msg: &Message, bytes: &mut[u8]) -> Result<usize> {
    /// #     Ok(0 as usize)
    /// #   }
    /// #   fn estimate_encoded_size(&mut self, msg: &Message) -> Result<usize> {
    /// #     Ok(0 as usize)
    /// #   }
    /// #   fn write_to(&mut self, msg: &Message, dest: &mut impl Write) -> Result<usize> {
    /// #     Ok(0 as usize)
    /// #   }
    /// #   fn reset_encoder(&mut self) {}
    /// # }
    /// # let mut channel = Channel::new(RplidarProtocol::new(), serial_port);
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
    /// ```rust
    /// # use std::time::Duration;
    /// # use rpos_drv::{ ProtocolDecoder, ProtocolEncoder, Channel, Message, Result, RingByteBuffer };
    /// # use std::io::{ Read, Write };
    /// # use std::fs::File;
    /// # let mut serial_port = Box::new(RingByteBuffer::with_capacity(100));
    /// # struct RplidarProtocol {}
    /// # impl RplidarProtocol {
    /// #   pub fn new() -> RplidarProtocol {
    /// #     RplidarProtocol {}
    /// #   }
    /// # }
    /// # impl ProtocolDecoder for RplidarProtocol {
    /// #   fn decode(&mut self, buf: &[u8]) -> Result<(usize, Option<Message>)> {
    /// #       Ok((0 as usize, None))
    /// #   }
    /// #   fn reset_decoder(&mut self) {}
    /// # }
    /// # impl ProtocolEncoder for RplidarProtocol {
    /// #   fn encode(&mut self, msg: &Message, bytes: &mut[u8]) -> Result<usize> {
    /// #     Ok(0 as usize)
    /// #   }
    /// #   fn estimate_encoded_size(&mut self, msg: &Message) -> Result<usize> {
    /// #     Ok(0 as usize)
    /// #   }
    /// #   fn write_to(&mut self, msg: &Message, dest: &mut impl Write) -> Result<usize> {
    /// #     Ok(0 as usize)
    /// #   }
    /// #   fn reset_encoder(&mut self) {}
    /// # }
    /// # let mut channel = Channel::new(RplidarProtocol::new(), serial_port);
    /// let resp = channel.invoke(&Message::new(1), Duration::from_secs(1));
    /// ```
    pub fn invoke(&mut self, request:&Message, timeout: Duration) -> Result<Option<Message>> {
        self.write(request)?;
        return self.read_until(timeout);
    }
    
}
