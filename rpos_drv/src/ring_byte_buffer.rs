use std::cmp::min;
use std::io::{Read, Write};

/// A ring byte buffer used to operate byte streams
/// 
/// # Example
/// 
/// ```rust
/// # use std::io::Write;
/// let mut buffer = rpos_drv::RingByteBuffer::with_capacity(100);
/// buffer.write(&[0, 1, 2, 3]).unwrap();
/// assert_eq!(buffer.len(), 4 as usize);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct RingByteBuffer {
    buf: Vec<u8>,
    head: usize,
    size: usize,
}

impl RingByteBuffer {
    /// create a ring buffer with capacity
    pub fn with_capacity(capacity: usize) -> RingByteBuffer {
        RingByteBuffer {
            buf: vec![0; capacity],
            head: 0,
            size: 0,
        }
    }

    /// the data in the ring buffer
    pub fn len(&self) -> usize {
        return self.size;
    }

    /// the capacity of the ring buffer
    pub fn capacity(&self) -> usize {
        return self.buf.len();
    }

    /// free space in bytes in the ring buffer
    pub fn free_space(&self) -> usize {
        return self.buf.len() - self.size;
    }

    /// current tail index of the ring buffer
    fn tail(&self) -> usize {
        return (self.head + self.size) % self.buf.len();
    }

    /// current read slice
    /// 
    /// # Example
    /// ```rust
    /// # use std::io::{ stdout, Write };
    /// # let mut buffer = rpos_drv::RingByteBuffer::with_capacity(100);
    /// # let mut some_stream = stdout();
    /// let read_slice = buffer.current_read_slice();
    /// let read = some_stream.write(read_slice).unwrap();
    /// buffer.skip_bytes(read);
    /// ```
    pub fn current_read_slice(&self) -> &[u8] {
        let end = min(self.head+self.size, self.buf.len());
        return &self.buf[self.head..end];
    }

    /// skip bytes
    pub fn skip_bytes(&mut self, bytes: usize) -> usize {
        let skipped = min(self.size, bytes);
        self.head = (self.head + skipped) % self.buf.len();
        self.size -= skipped;
        return skipped;
    }

    /// current write slice
    fn current_write_slice(&mut self) -> &mut[u8] {
        let current_end = self.tail();
        let write_buf_end = min(self.buf.len(), current_end + self.free_space());
        return &mut self.buf[current_end..write_buf_end];
    }

    /// mark bytes as written
    fn mark_bytes_as_written(&mut self, bytes: usize) {
        let written = min(self.free_space(), bytes);
        self.size += written;
    }

    fn partial_read_from(&mut self, upstream: &mut impl Read) -> std::io::Result<usize> {
        if self.current_write_slice().len() == 0 {
            return Ok(0);
        }

        match upstream.read(self.current_write_slice()) {
            Ok(read) => {
                self.mark_bytes_as_written(read);
                Ok(read)
            },
            Err(err) => {
                if err.kind() == std::io::ErrorKind::TimedOut {
                    Ok(0)
                } else {
                    Err(err)
                }
            }
        }
    }

    /// read data from upstream to fill the ring buffer
    /// 
    /// # Example
    /// ```rust
    /// # use std::io::{ stdin, Read };
    /// # let mut buffer = rpos_drv::RingByteBuffer::with_capacity(100);
    /// # let mut some_stream = stdin();
    /// let read = buffer.read_from(&mut some_stream).unwrap();
    /// println!("{} bytes read from stream to fill buffer, current length of buffer is: {}", read, buffer.len());
    /// ```
    pub fn read_from(&mut self, upstream: &mut impl Read) -> std::io::Result<usize> {
        let read = self.partial_read_from(upstream)?;

        match self.partial_read_from(upstream) {
            Ok(latter_read) => Ok(read + latter_read),
            Err(err) => Err(err)
        }
    }
}

impl Read for RingByteBuffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let read = {
            let current_read_slice = self.current_read_slice();
            let read = min(current_read_slice.len(), buf.len());
            buf[0..read].clone_from_slice(&current_read_slice[0..read]);

            read
        };
        self.skip_bytes(read);

        let latter_read = {
            let current_read_slice = self.current_read_slice();
            let latter_read = min(current_read_slice.len(), buf.len() - read);
            buf[read..read + latter_read].clone_from_slice(&current_read_slice[0..latter_read]);

            latter_read
        };
        self.skip_bytes(latter_read);

        return Ok(read + latter_read);
    }
}

impl Write for RingByteBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let written = {
            let current_write_slice = self.current_write_slice();
            let written = min(current_write_slice.len(), buf.len());
            current_write_slice[0..written].clone_from_slice(&buf[0..written]);
            written
        };
        self.mark_bytes_as_written(written);

        let latter_written = {
            let current_write_slice = self.current_write_slice();
            let latter_written = min(current_write_slice.len(), buf.len() - written);
            current_write_slice[0..latter_written].clone_from_slice(&buf[written..written + latter_written]);
            latter_written
        };
        self.mark_bytes_as_written(latter_written);

        return Ok(written + latter_written);
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    #[test]
    fn constructor() {
        let ring_buf = super::RingByteBuffer::with_capacity(100);

        assert_eq!(ring_buf.len(), 0);
        assert_eq!(ring_buf.capacity(), 100);
        assert_eq!(ring_buf.free_space(), 100);
        assert_eq!(ring_buf.current_read_slice().len(), 0);
    }

    #[test]
    fn read_and_write() {
        let mut ring_buf = super::RingByteBuffer::with_capacity(6);

        let test_data = vec![1,2,3,4];

        assert_eq!(ring_buf.write(&test_data[..]).unwrap(), 4);
        assert_eq!(ring_buf.len(), 4);
        assert_eq!(ring_buf.free_space(), 2);

        assert_eq!(ring_buf.write(&test_data[..]).unwrap(), 2);
        assert_eq!(ring_buf.len(), 6);
        assert_eq!(ring_buf.free_space(), 0);

        assert_eq!(ring_buf.current_read_slice(), [1, 2, 3, 4, 1, 2]);

        let mut read_buf = [0; 5];

        assert_eq!(ring_buf.read(&mut read_buf).unwrap(), 5);
        assert_eq!(read_buf, [1, 2, 3, 4, 1]);
        assert_eq!(ring_buf.len(), 1);
        assert_eq!(ring_buf.free_space(), 5);
        assert_eq!(ring_buf.read(&mut read_buf).unwrap(), 1);
        assert_eq!(read_buf, [2, 2, 3, 4, 1]);
        assert_eq!(ring_buf.len(), 0);
        assert_eq!(ring_buf.free_space(), 6);
    }
}
