use core::ptr;

use crate::buffer::Buffer;
use crate::impls::utils::Separator;

#[inline(always)]
pub(crate) fn write_two_bytes(
    buf: &mut Buffer,
    sep: &mut Option<Separator>,
    table_ptr: *const u8,
    table_index: isize,
) {
    for i in (0..2).rev() {
        write_one_byte(buf, sep, table_ptr, table_index + i);
    }
}

#[inline(always)]
pub(crate) fn write_one_byte(
    buf: &mut Buffer,
    sep: &mut Option<Separator>,
    table_ptr: *const u8,
    table_index: isize,
) {
    buf.pos -= 1;
    if let Some(ref mut sep) = sep {
        if sep.pos == (buf.pos as isize) {
            let sep_ptr = sep.ptr;
            let sep_len = sep.len;
            buf.pos -= sep_len - 1;
            unsafe { ptr::copy_nonoverlapping(sep_ptr, buf.as_mut_ptr().add(buf.pos), sep_len) }
            sep.pos -= sep.step + (sep_len as isize - 1);
            buf.pos -= 1;
        }
    }
    unsafe {
        ptr::copy_nonoverlapping(
            table_ptr.offset(table_index),
            buf.as_mut_ptr().add(buf.pos),
            1,
        )
    };
}
