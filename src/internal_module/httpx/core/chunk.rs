use std::{
    io::{BufRead, Cursor},
    ops::Index,
};

use super::ParseError;

pub struct HttpChunk;

impl HttpChunk {
    pub fn parse(buf: &[u8]) -> Result<(&[u8], usize), ParseError> {
        let mut c = Cursor::new(buf);
        let mut header = String::new();
        let n = c.read_line(&mut header).map_err(|e| {
            println!("header {:?}", e);
            ParseError::InvalidChunk
        })?;
        if n == 0 {
            return Err(ParseError::Pending);
        }

        let len = usize::from_str_radix(header.trim(), 16).map_err(|e| {
            println!("hex{} {:?}", header.trim(), e);
            ParseError::InvalidChunk
        })?;

        let r_buf = buf.get(n..n + len).ok_or(ParseError::Pending)?;

        c.set_position((n + len) as u64);

        let mut end = header;
        let end_n = c.read_line(&mut end).map_err(|e| {
            println!("end_n {:?}", e);
            ParseError::InvalidChunk
        })?;

        if end_n == 0 {
            return Err(ParseError::Pending);
        }

        Ok((r_buf, n + len + end_n))
    }
}
