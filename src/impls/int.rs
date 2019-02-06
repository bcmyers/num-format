use core::mem;
use core::ptr;

use itoa;

use crate::constants::{MAX_BUF_LEN, TABLE};
use crate::sealed::Sealed;
use crate::utils::{self, Separator};
use crate::{Buffer, Format, ToFormattedStr};

macro_rules! impl_signed {
    ($type:ty, $max:expr) => {
        impl ToFormattedStr for $type {
            #[doc(hidden)]
            #[inline(always)]
            fn read_to_buffer<'a, F>(&self, buf: &'a mut Buffer, format: &F) -> usize
            where
                F: Format,
            {
                // Encode the separator, if any, into a byte buffer
                let mut sep_buf: [u8; 4] = unsafe { mem::uninitialized() };
                let mut sep = Separator::new(format, &mut sep_buf);

                // Figure out if we are negative and what our minus sign is
                let is_negative = self.is_negative();
                let min = format.minus_sign().into_str();

                // Bail out early if we can just use itoa
                // (i.e. if we don't have a separator and a minus sign doesn't cause us problems)
                if sep.is_none() && ((is_negative && min == "-") || !is_negative) {
                    let c = itoa::write(&mut buf.inner[..], *self).unwrap();
                    buf.pos = 0;
                    buf.end = c;
                    return c;
                }

                // Reset our position to the end of the buffer
                buf.pos = MAX_BUF_LEN;
                buf.end = MAX_BUF_LEN;

                // Get a pointer to TABLE, which will be needed later
                let table_ptr = TABLE.as_ptr();

                // Turn ourselves into a postive u128
                let mut n = if is_negative {
                    (!(*self as u128)).wrapping_add(1) // make positive by adding 1 to the 2s complement
                } else {
                    *self as u128
                };

                // Start the main algorith
                while n >= 10_000 {
                    let remainder = n % 10_000;
                    let table_index = ((remainder % 100) << 1) as isize;
                    utils::write_two_bytes(buf, &mut sep, table_ptr, table_index);
                    let table_index = ((remainder / 100) << 1) as isize;
                    utils::write_two_bytes(buf, &mut sep, table_ptr, table_index);
                    n /= 10_000;
                }
                let mut n = n as isize;
                while n >= 100 {
                    let table_index = (n % 100) << 1;
                    utils::write_two_bytes(buf, &mut sep, table_ptr, table_index);
                    n /= 100;
                }
                if n >= 10 {
                    let table_index = n << 1;
                    utils::write_two_bytes(buf, &mut sep, table_ptr, table_index);
                } else {
                    let table_index = n << 1;
                    utils::write_one_byte(buf, &mut sep, table_ptr, table_index + 1);
                }

                // Add on the minus sign if we are negative
                if is_negative {
                    let min_len = min.len();
                    buf.pos -= min_len;
                    let min_ptr = min.as_bytes().as_ptr();
                    unsafe {
                        ptr::copy_nonoverlapping(min_ptr, buf.as_mut_ptr().add(buf.pos), min_len)
                    }
                }

                buf.end - buf.pos
            }
        }

        impl Sealed for $type {}
    };
}

impl_signed!(i8, std::i8::MAX);
impl_signed!(i16, std::i16::MAX);
impl_signed!(i32, std::i32::MAX);
impl_signed!(isize, std::isize::MAX);
impl_signed!(i64, std::i64::MAX);
impl_signed!(i128, std::i128::MAX);
