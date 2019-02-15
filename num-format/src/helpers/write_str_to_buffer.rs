use crate::Grouping;

pub(crate) fn write_str_to_buffer<'a>(
    buf: &'a mut Vec<u8>,
    grp: Grouping,
    s: &str,
    sep: &str,
) -> usize {
    let mut buf_pos = buf.len();
    let mut next_sep = buf_pos.checked_sub(4);

    let sep_len = sep.len();

    for c in s.chars().rev() {
        buf_pos -= 1;
        if let Some(sep_pos) = next_sep {
            if buf_pos == sep_pos {
                buf_pos -= sep_len - 1;
                buf[buf_pos..buf_pos + sep_len].copy_from_slice(sep.as_bytes());
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
