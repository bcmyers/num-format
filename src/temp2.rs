const DEC_DIGITS_LUT: &'static [u8] = b"\
      0001020304050607080910111213141516171819\
      2021222324252627282930313233343536373839\
      4041424344454647484950515253545556575859\
      6061626364656667686970717273747576777879\
      8081828384858687888990919293949596979899";

impl OtherIntegers for u8 {
    #[allow(unused_comparisons)]
    #[inline]
    fn write_to(self, buf: &mut [u8; MAX_LEN_OF_TYPE]) -> &[u8] {
        let is_nonnegative = self >= 0;
        let mut n = if is_nonnegative {
            self as CONV_FN
        } else {
            // convert the negative num to positive by summing 1 to it's 2 complement
            (!(self as CONV_FN)).wrapping_add(1)
        };
        let mut curr = buf.len() as isize;
        let buf_ptr = buf.as_mut_ptr();
        let lut_ptr = DEC_DIGITS_LUT.as_ptr();

        unsafe {
            // need at least 16 bits for the 4-characters-at-a-time to work.
            if mem::size_of::<u8>() >= 2 {
                // eagerly decode 4 characters at a time
                while n >= 10000 {
                    let rem = (n % 10000) as isize;
                    n /= 10000;

                    let d1 = (rem / 100) << 1;
                    let d2 = (rem % 100) << 1;
                    curr -= 4;
                    ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
                    ptr::copy_nonoverlapping(lut_ptr.offset(d2), buf_ptr.offset(curr + 2), 2);
                }
            }

            // if we reach here numbers are <= 9999, so at most 4 chars long
            let mut n = n as isize; // possibly reduce 64bit math

            // decode 2 more chars, if > 2 chars
            if n >= 100 {
                let d1 = (n % 100) << 1;
                n /= 100;
                curr -= 2;
                ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
            }

            // decode last 1 or 2 chars
            if n < 10 {
                curr -= 1;
                *buf_ptr.offset(curr) = (n as u8) + b'0';
            } else {
                let d1 = n << 1;
                curr -= 2;
                ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
            }

            if !is_nonnegative {
                curr -= 1;
                *buf_ptr.offset(curr) = b'-';
            }
        }

        let len = buf.len() - curr as usize;
        unsafe { slice::from_raw_parts(buf_ptr.offset(curr), len) }
    }
}

impl Integer128 for i128 {
    #[allow(unused_comparisons)]
    #[inline]
    fn write_to(self, buf: &mut [u8; MAX_LEN_FOR_TYPE]) -> &[u8] {
        let is_nonnegative = self >= 0;
        let n = if is_nonnegative {
            self as u128
        } else {
            // convert the negative num to positive by summing 1 to it's 2 complement
            (!(self as u128)).wrapping_add(1)
        };
        let mut curr = buf.len() as isize;
        let buf_ptr = buf.as_mut_ptr();

        unsafe {
            // Divide by 10^19 which is the highest power less than 2^64.
            let (n, rem) = udiv128::udivmod_1e19(n);
            let buf1 = buf_ptr.offset(curr - U64_MAX_LEN as isize) as *mut [u8; U64_MAX_LEN];
            curr -= rem.write_to(&mut *buf1).len() as isize;

            if n != 0 {
                // Memset the base10 leading zeros of rem.
                let target = buf.len() as isize - 19;
                ptr::write_bytes(buf_ptr.offset(target), b'0', (curr - target) as usize);
                curr = target;

                // Divide by 10^19 again.
                let (n, rem) = udiv128::udivmod_1e19(n);
                let buf2 = buf_ptr.offset(curr - U64_MAX_LEN as isize) as *mut [u8; U64_MAX_LEN];
                curr -= rem.write_to(&mut *buf2).len() as isize;

                if n != 0 {
                    // Memset the leading zeros.
                    let target = buf.len() as isize - 38;
                    ptr::write_bytes(buf_ptr.offset(target), b'0', (curr - target) as usize);
                    curr = target;

                    // There is at most one digit left
                    // because u128::max / 10^19 / 10^19 is 3.
                    curr -= 1;
                    *buf_ptr.offset(curr) = (n as u8) + b'0';
                }
            }

            if !is_nonnegative {
                curr -= 1;
                *buf_ptr.offset(curr) = b'-';
            }

            let len = buf.len() - curr as usize;
            slice::from_raw_parts(buf_ptr.offset(curr), len)
        }
    }
}
