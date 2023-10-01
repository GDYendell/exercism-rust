use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    reader: R,
    total_reads: usize,
    total_bytes_read: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            reader: wrapped,
            total_reads: 0,
            total_bytes_read: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.total_bytes_read
    }

    pub fn reads(&self) -> usize {
        self.total_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let read_result = self.reader.read(buf);
        match read_result {
            Ok(bytes_read) => {
                self.total_reads += 1;
                self.total_bytes_read += bytes_read;
                Ok(bytes_read)
            }
            Err(e) => Err(e),
        }
    }
}

pub struct WriteStats<W> {
    writer: W,
    total_writes: usize,
    total_bytes_written: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            writer: wrapped,
            total_writes: 0,
            total_bytes_written: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.total_bytes_written
    }

    pub fn writes(&self) -> usize {
        self.total_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let write_result = self.writer.write(buf);
        match write_result {
            Ok(bytes_written) => {
                self.total_writes += 1;
                self.total_bytes_written += bytes_written;
                Ok(bytes_written)
            }
            Err(e) => Err(e),
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
