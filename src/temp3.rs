use std::ptr;
use std::slice;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_me() {
        let s = to_string(1000);
        println!("{}", &s);
    }
}

const TABLE: &'static [u8] = b"\
      0001020304050607080910111213141516171819\
      2021222324252627282930313233343536373839\
      4041424344454647484950515253545556575859\
      6061626364656667686970717273747576777879\
      8081828384858687888990919293949596979899";

pub fn to_string(mut n: u32) -> String {
    // max size is 54 = 32 (digits) + 12 (separators) + 3 (minus sign)
    let is_positive = false; // todo
    let mut buf = [',' as u8; 54];
    let ptr = buf.as_mut_ptr();
    let table = TABLE.as_ptr();
    let len = buf.len();
    let mut pos = len as isize;
    let mut written = 0usize;

    unsafe {
        while n >= 10_000 {
            let rem = (n % 10_000) as isize;
            n /= 10_000;

            let d1 = (rem / 100) << 1;
            let d2 = (rem % 100) << 1;

            pos -= 1;
            if (len - pos as usize) % 4 == 0 {
                pos -= 1;
            }
            ptr::copy_nonoverlapping(table.offset(d2 + 1), ptr.offset(pos), 1);
            pos -= 1;
            if (len - pos as usize) % 4 == 0 {
                pos -= 1;
            }
            ptr::copy_nonoverlapping(table.offset(d2), ptr.offset(pos), 1);
            pos -= 1;
            if (len - pos as usize) % 4 == 0 {
                pos -= 1;
            }
            ptr::copy_nonoverlapping(table.offset(d1 + 1), ptr.offset(pos), 1);
            pos -= 1;
            if (len - pos as usize) % 4 == 0 {
                pos -= 1;
            }
            ptr::copy_nonoverlapping(table.offset(d1), ptr.offset(pos), 1);
        }

        // if we reach here numbers are <= 9999, so at most 4 chars long
        let mut n = n as isize; // possibly reduce 64bit math

        // decode 2 more chars, if > 2 chars
        if n >= 100 {
            let d1 = (n % 100) << 1;
            n /= 100;
            pos -= 1;
            if (len - pos as usize) % 4 == 0 {
                pos -= 1;
            }
            ptr::copy_nonoverlapping(table.offset(d1 + 1), ptr.offset(pos), 1);
            pos -= 1;
            if (len - pos as usize) % 4 == 0 {
                pos -= 1;
            }
            ptr::copy_nonoverlapping(table.offset(d1), ptr.offset(pos), 1);
        }

        // decode last 1 or 2 chars
        if n < 10 {
            pos -= 1;
            *ptr.offset(pos) = (n as u8) + b'0';
        } else {
            let d1 = n << 1;
            pos -= 1;
            if (len - pos as usize) % 4 == 0 {
                pos -= 1;
            }
            ptr::copy_nonoverlapping(table.offset(d1 + 1), ptr.offset(pos), 1);
            pos -= 1;
            if (len - pos as usize) % 4 == 0 {
                pos -= 1;
            }
            ptr::copy_nonoverlapping(table.offset(d1), ptr.offset(pos), 1);
        }

        if !is_positive {
            pos -= 1;
            *ptr.offset(pos) = b'-';
        }

        let slice_len = len - pos as usize;
        let x = slice::from_raw_parts(ptr.offset(pos), slice_len);
        String::from_utf8_unchecked(x.to_vec())
    }
}

#[inline]
fn len_new(len_old: usize, no_sep: usize, len_sep: usize, len_minus: usize) -> usize {
    len_old + no_sep * len_sep + len_minus
}

#[inline]
fn no_sep(len_old: usize) -> usize {
    match len_old % 3 {
        0 => len_old / 3 - 1,
        _ => len_old / 3,
    }
}

#[inline]
fn len_old(mut x: u128) -> usize {
    let mut count = 0;
    while x != 0 {
        x = x / 10;
        count += 1;
    }
    count
}
