use core::mem;
use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};

use crate::constants::{MAX_BUF_LEN, TABLE};
use crate::helpers::{self, Separator};
use crate::sealed::Sealed;
use crate::{Buffer, Format, ToFormattedStr};

impl ToFormattedStr for NonZeroU8 {
    #[doc(hidden)]
    #[inline(always)]
    fn read_to_buffer<'a, F>(&self, buf: &'a mut Buffer, _: &F) -> usize
    where
        F: Format,
    {
        buf.write_with_itoa(self.get())
    }
}

impl Sealed for NonZeroU8 {}

macro_rules! impl_non_zero {
    ($type:ty) => {
        impl ToFormattedStr for $type {
            #[doc(hidden)]
            #[inline(always)]
            fn read_to_buffer<'a, F>(&self, buf: &'a mut Buffer, format: &F) -> usize
            where
                F: Format,
            {
                // Encode the separator, if any, into a byte buffer
                let mut sep_buf: [u8; 4] = unsafe { mem::uninitialized() };
                let mut sep: Option<Separator> = Separator::new(format, &mut sep_buf);

                // Bail out early if we can just use itoa
                // (i.e. if we don't have a separator)
                if sep.is_none() {
                    return buf.write_with_itoa(self.get());
                }

                // Reset our position to the end of the buffer
                buf.pos = MAX_BUF_LEN;
                buf.end = MAX_BUF_LEN;

                // Get a pointer to TABLE, which will be needed later
                let table_ptr = TABLE.as_ptr();

                // Start the main algorithm
                #[allow(trivial_numeric_casts)]
                let mut n = self.get() as u128;
                while n >= 10_000 {
                    let remainder = n % 10_000;
                    let table_index = ((remainder % 100) << 1) as isize;
                    helpers::write_two_bytes(buf, &mut sep, table_ptr, table_index);
                    let table_index = ((remainder / 100) << 1) as isize;
                    helpers::write_two_bytes(buf, &mut sep, table_ptr, table_index);
                    n /= 10_000;
                }
                let mut n = n as isize;
                while n >= 100 {
                    let table_index = (n % 100) << 1;
                    helpers::write_two_bytes(buf, &mut sep, table_ptr, table_index);
                    n /= 100;
                }
                if n >= 10 {
                    let table_index = n << 1;
                    helpers::write_two_bytes(buf, &mut sep, table_ptr, table_index);
                } else {
                    let table_index = n << 1;
                    helpers::write_one_byte(buf, &mut sep, table_ptr, table_index + 1);
                }
                buf.end - buf.pos
            }
        }

        impl Sealed for $type {}
    };
}

impl_non_zero!(NonZeroU16);
impl_non_zero!(NonZeroU32);
impl_non_zero!(NonZeroUsize);
impl_non_zero!(NonZeroU64);
impl_non_zero!(NonZeroU128);
