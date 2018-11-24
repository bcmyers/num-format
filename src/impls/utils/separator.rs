use crate::constants::MAX_BUF_LEN;
use crate::format::{Format, Grouping};

#[derive(Debug)]
pub(crate) struct Separator {
    pub(crate) len: usize,
    pub(crate) ptr: *const u8,
    pub(crate) pos: isize,
    pub(crate) step: isize,
}

impl Separator {
    #[inline(always)]
    pub(crate) fn new<F>(format: &F, sep_buf: &mut [u8]) -> Option<Separator>
    where
        F: Format,
    {
        match (format.grouping(), format.separator()) {
            (Grouping::Standard, Some(c)) => {
                let _ = c.encode_utf8(sep_buf);
                Some(Separator {
                    len: c.len_utf8(),
                    ptr: sep_buf.as_ptr(),
                    pos: MAX_BUF_LEN as isize - 4,
                    step: 4,
                })
            }
            (Grouping::Indian, Some(c)) => {
                let _ = c.encode_utf8(sep_buf);
                Some(Separator {
                    len: c.len_utf8(),
                    ptr: sep_buf.as_ptr(),
                    pos: MAX_BUF_LEN as isize - 4,
                    step: 3,
                })
            }
            _ => None,
        }
    }
}
