pub(crate) use num_format_core::constants::{MAX_ERR_LEN, MAX_INF_LEN, MAX_MIN_LEN, MAX_NAN_LEN};

// Want this to be as large as the largest possible string representation of any type
// that implements ToFormattedStr, which is currently i128's Grouping::Indian representation.
// The max len of an i128 formatted string is ...
// 39 digits + 18 separators (each potentially 4 bytes) + 1 minus sign (potentially 7 bytes)
pub(crate) const MAX_BUF_LEN: usize = 39 + 18 * 4 + MAX_MIN_LEN;

pub(crate) const TABLE: &[u8] = b"\
    0001020304050607080910111213141516171819\
    2021222324252627282930313233343536373839\
    4041424344454647484950515253545556575859\
    6061626364656667686970717273747576777879\
    8081828384858687888990919293949596979899";
