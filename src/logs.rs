use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;

// 10 KB = 10240
const BUFSIZE: u64 = 10240;

pub struct LogReader {
    fp: File,
    size: u64,
}

impl LogReader {
    pub fn new(filename: String) -> Self {
        let fp = File::open(filename).unwrap();
        let size = fp.metadata().unwrap().len();

        LogReader {
            fp,
            size,
        }
    }

    pub fn iter(&mut self) -> RevLogIter {
        let pos = self.size;
        RevLogIter {
            log_reader: self,
            buffered: vec![],
            pos,
        }
    }
}

pub struct RevLogIter<'a> {
    log_reader: &'a mut LogReader,
    buffered: Vec<String>,
    pos: u64,
}

/// Go backward through the file, reading 'BUFSIZE' bytes at a time (except
/// probably the first), until we hit the start of the file or have
/// read NUMBER newlines.
/// START_POS is the starting position of the read pointer for the file
/// associated with FD (may be nonzero).
/// END_POS is the file offset of EOF (one larger than offset of last byte).
/// Return true if successful.
/// 
/// Adapted from tail:
/// https://git.savannah.gnu.org/cgit/coreutils.git/tree/src/tail.c#n525
impl<'a> RevLogIter<'a> {

    /// Fills the buffer with the next lines that take up BUFSIZE space.
    /// If there line is longer than BUFSIZE, keep reading in BUFSIZE chunks
    /// until a newline is reached. If the new start position isn't 0, set
    /// the new position to the offset + the length of the incomplete line.
    fn fill_buffer(&mut self) {
        let mut buf = String::new();
        while buf.matches("\n").count() < 2 && self.pos != 0 {
            // Set the new starting position to either the size of the buffer
            // or the start of the file.
            let mut offset = BUFSIZE;
            self.pos = if self.pos < BUFSIZE {
                offset += BUFSIZE - self.pos ;
                0
            } else {
                self.pos - BUFSIZE
            };

            // Seek to the new start position.
            self.log_reader.fp
                .seek(std::io::SeekFrom::Start(self.pos))
                .unwrap();

            let mut cur_buf  = String::new();
            // Read to buffer.
            self.log_reader.fp.try_clone()
                .unwrap()
                .take(offset)
                .read_to_string(&mut cur_buf)
                .unwrap();
            buf = cur_buf.add(buf.as_str());
        }
        let mut lines = buf.lines()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();

        if self.pos != 0 {
            // If self.pos != 0, remove the first line from the buffer and add it's
            // length back to self.pos. Otherwise, If we're at the top if the file,
            // we collect the top line.
            self.pos += 1 + lines.remove(0).len() as u64;
        }

        self.buffered.append(&mut lines);
    }
}

impl<'a> Iterator for RevLogIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.buffered.is_empty() {
            return self.buffered.pop();
        }
        
        self.fill_buffer();
        self.buffered.pop()
    }
}