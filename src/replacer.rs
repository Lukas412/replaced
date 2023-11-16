use std::io::{Read, Write};

pub struct Replacer<Buffer, Reader, Comparer> {
    buffer: Buffer,
    reader: Reader,
    comparer: Comparer,
}

impl<Reader, Comparer> Replacer<String, Reader, Comparer>
where
    Reader: Read,
{
    pub fn dynamic_buffer(reader: Reader, comparer: Comparer) -> Self {
        Self {
            buffer: String::new(),
            reader,
            comparer,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

impl<Buffer, Reader> Replacer<Buffer, Reader> {}

impl<Buffer, Reader> Write for Replacer<Buffer, Reader> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {}

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
