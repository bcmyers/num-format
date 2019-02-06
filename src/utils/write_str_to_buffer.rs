use core::mem;

use crate::Grouping;

pub(crate) fn write_str_to_buffer<'a>(
    buf: &'a mut Vec<u8>,
    grp: Grouping,
    s: &str,
    sep: char,
) -> usize {
    let mut buf_pos = buf.len();
    let mut next_sep = buf_pos.checked_sub(4);

    let mut sep_buf: [u8; 4] = unsafe { mem::uninitialized() };
    let sep_str = sep.encode_utf8(&mut sep_buf);
    let sep_len = sep_str.len();

    for c in s.chars().rev() {
        buf_pos -= 1;
        if let Some(sep_pos) = next_sep {
            if buf_pos == sep_pos {
                buf_pos -= sep_len - 1;
                buf[buf_pos..buf_pos + sep_len].copy_from_slice(sep_str.as_bytes());
                next_sep = buf_pos.checked_sub(match grp {
                    Grouping::Standard => 4,
                    Grouping::Indian => 3,
                    Grouping::Posix => unreachable!(),
                });
                buf_pos -= 1;
            }
        }
        buf[buf_pos] = c as u8;
    }

    buf_pos
}
