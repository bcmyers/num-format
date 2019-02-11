use std::fmt;
use std::io::{self, Error, ErrorKind};
use std::str;

use num_bigint::BigUint;

use crate::constants::MAX_MIN_LEN;
use crate::helpers::write_str_to_buffer;
use crate::sealed::Sealed;
use crate::{Format, Grouping, ToFormattedString};

impl ToFormattedString for BigUint {
    fn read_to_io_writer<F, W>(&self, mut w: W, format: &F) -> Result<usize, io::Error>
    where
        F: Format,
        W: io::Write,
    {
        let sep = format.separator();
        let grp = format.grouping();

        if sep.is_none() || grp == Grouping::Posix {
            // If we can just use BigInt's to_string method, let's do it
            let s = self.to_string();
            w.write_all(s.as_bytes())?;
            return Ok(s.len());
        }

        let s = self.to_string();
        let sep = sep.unwrap();

        // Create the buffer
        let buf_len = match s.len().checked_mul(3) {
            Some(v) => v,
            None => s.len(),
        };
        let mut buf: Vec<u8> = Vec::with_capacity(buf_len);
        unsafe { buf.set_len(buf_len) };

        // Write to the buffer
        let buf_pos = write_str_to_buffer(&mut buf, grp, &s, sep);

        // Wrap up
        w.write_all(&buf[buf_pos..])?;

        Ok(buf_len - buf_pos)
    }

    fn read_to_fmt_writer<F, W>(&self, mut w: W, format: &F) -> Result<usize, io::Error>
    where
        F: Format,
        W: fmt::Write,
    {
        let sep = format.separator();
        let grp = format.grouping();

        if sep.is_none() || grp == Grouping::Posix {
            // If we can just use BigInt's to_string method, let's do it
            let s = self.to_string();
            w.write_str(&s)
                .map_err(|e| Error::new(ErrorKind::Other, e))?;
            return Ok(s.len());
        }

        let s = self.to_string();
        let sep = sep.unwrap();

        // Create the buffer
        let buf_len = match s.len().checked_mul(3) {
            Some(v) => v + MAX_MIN_LEN,
            None => s.len(),
        };
        let mut buf: Vec<u8> = Vec::with_capacity(buf_len);
        unsafe { buf.set_len(buf_len) };

        // Write to the buffer
        let buf_pos = write_str_to_buffer(&mut buf, grp, &s, sep);

        // Wrap up
        let s = unsafe { str::from_utf8_unchecked(&buf[buf_pos..]) };
        w.write_str(s)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        Ok(buf_len - buf_pos)
    }
}

impl Sealed for BigUint {}
