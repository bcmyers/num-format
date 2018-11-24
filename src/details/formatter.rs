use crate::details::{Grouping, Number};

pub const MINUS_SIGN_MAX_BYTES: usize = 3;

pub struct Formatter<'a> {
    kind: Number,
    decimal: char,
    decimal_places: usize,
    grouping: Grouping,
    infinity: &'a str,
    minus_sign: char,
    nan: &'a str,
    percent_sign: &'a str,
    separator: char,
}

impl<'a> Formatter<'a> {
    pub fn to_string(&self) -> String {
        match self.kind {
            Number::Integer { ref s, is_positive } => self.to_string_integer(s, is_positive),
            Number::FloatNormal { ref s, is_positive } => self.to_string_float(s, is_positive),
            Number::FloatInfinity { is_positive } => {
                if is_positive {
                    self.infinity.to_string()
                } else {
                    let mut minus_sign_buf = [0u8; 4];
                    self.minus_sign.encode_utf8(&mut minus_sign_buf);
                    let mut buf = Vec::with_capacity(
                        self.minus_sign.len_utf8() + self.infinity.as_bytes().len(),
                    );
                    buf.extend_from_slice(&minus_sign_buf[0..self.minus_sign.len_utf8()]);
                    buf.extend_from_slice(self.infinity.as_bytes());
                    unsafe { String::from_utf8_unchecked(buf) }
                }
            }
            Number::FloatNan => self.nan.to_string(),
        }
    }

    pub fn to_percent(&self) -> String {
        match self.kind {
            Number::FloatNormal { ref s, is_positive } => {
                let _ = s;
                let _ = is_positive;
                unimplemented!()
            }
            Number::Integer { .. } => unimplemented!(),
            _ => self.to_string(),
        }
    }

    pub fn decimal(mut self, value: char) -> Formatter<'a> {
        self.decimal = value;
        self
    }

    pub fn decimal_places(mut self, value: usize) -> Formatter<'a> {
        self.decimal_places = value;
        self
    }

    pub fn grouping(mut self, value: Grouping) -> Formatter<'a> {
        self.grouping = value;
        self
    }

    pub fn infinity(mut self, value: &'a str) -> Formatter<'a> {
        self.infinity = value;
        self
    }

    pub fn minus_sign(mut self, value: char) -> Formatter<'a> {
        self.minus_sign = value;
        self
    }

    pub fn nan(mut self, value: &'a str) -> Formatter<'a> {
        self.nan = value;
        self
    }

    pub fn percent_sign(mut self, value: &'a str) -> Formatter<'a> {
        self.percent_sign = value;
        self
    }

    pub fn separator(mut self, value: char) -> Formatter<'a> {
        self.separator = value;
        self
    }

    // Helper methods...

    #[inline]
    fn to_string_float(&self, s: &str, is_positive: bool) -> String {
        let _ = s;
        let _ = is_positive;
        unimplemented!()
    }

    #[inline]
    fn to_string_integer(&self, s: &str, is_positive: bool) -> String {
        let old = s.as_bytes();
        let len_old = old.len();
        let len_new = self.len_integer(s, is_positive);
        debug_assert!(len_new <= std::isize::MAX as usize);

        let mut new = Vec::with_capacity(len_new);
        unsafe { new.set_len(len_new) };

        let mut pos_old = len_old as isize - 1;
        let mut pos_new = len_new as isize - 1;
        let mut pos_next_sep = pos_new as isize - 3;

        if is_positive {
            // Loop from largest position to the smallest position
            while pos_new >= 0 {
                if pos_new == pos_next_sep {
                    let mut char_buf = [0u8; 4];
                    self.separator.encode_utf8(&mut char_buf);
                    let len = self.separator.len_utf8();
                    for i in (0..len).rev() {
                        new[pos_new as usize] = char_buf[i];
                        pos_new -= 1;
                    }
                    pos_next_sep = pos_new - 3;
                    continue;
                }
                new[pos_new as usize] = old[pos_old as usize];
                pos_new -= 1;
                pos_old -= 1;
            }
        } else {
            // Loop from largest position to the smallest position,
            // but skip the very smallest positions where the minus
            // sign goes
            let len_minus = self.minus_sign.len_utf8();
            while pos_new >= len_minus as isize {
                if pos_new == pos_next_sep {
                    let mut char_buf = [0u8; 4];
                    self.separator.encode_utf8(&mut char_buf);
                    let len = self.separator.len_utf8();
                    for i in (0..len).rev() {
                        new[pos_new as usize] = char_buf[i];
                        pos_new -= 1;
                    }
                    pos_next_sep = pos_new - 3;
                    continue;
                }
                new[pos_new as usize] = old[pos_old as usize];
                pos_new -= 1;
                pos_old -= 1;
            }
            // Now place the minus sign
            let mut char_buf = [0u8; 4];
            self.minus_sign.encode_utf8(&mut char_buf);
            for i in 0..len_minus {
                new[i] = char_buf[i];
            }
        }
        unsafe { String::from_utf8_unchecked(new) }
    }

    #[inline]
    fn len_integer(&self, s: &str, is_positive: bool) -> usize {
        let len_old = s.as_bytes().len();
        let len_old_ex_minus = if is_positive { len_old } else { len_old - 1 };
        let len_minus = if is_positive {
            0
        } else {
            self.minus_sign.len_utf8()
        };
        let len_sep = self.separator.len_utf8();
        let no_sep = (len_old_ex_minus - 1) / 3;
        len_old_ex_minus + len_minus + no_sep * len_sep
    }
}

impl From<Number> for Formatter<'static> {
    fn from(number: Number) -> Formatter<'static> {
        Formatter {
            kind: number,
            decimal: '.',
            decimal_places: 2,
            grouping: Grouping::Standard,
            infinity: "∞",
            minus_sign: '-',
            nan: "NaN",
            percent_sign: "%",
            separator: ',',
        }
    }
}
