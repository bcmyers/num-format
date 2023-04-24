use core::borrow::Borrow;
use core::fmt;
use core::ops::Deref;
use core::str;

use crate::constants::MAX_BUF_LEN;
use crate::format::Format;
use crate::to_formatted_str::ToFormattedStr;

/// <b><u>A key type</u></b>. Represents a stack-allocated buffer you can use to get a
/// formatted `&str` without heap allocation.
///
/// # Example
/// ```
/// use num_format::{Buffer, Locale};
///
/// fn main() {
///     // Create a stack-allocated buffer...
///     let mut buf = Buffer::default();
///
///     // Write "1,000,000" into the buffer...
///     buf.write_formatted(&1000000, &Locale::en);
///
///     // Get a view into the buffer as a &str...
///     let s = buf.as_str();
///
///     // Do what you want with the &str...
///     assert_eq!("1,000,000", s);
///
///     // No need to clear the buffer before further calls to `write_formatted`...
///     buf.write_formatted(&1000000, &Locale::fr);
///     assert_eq!("1\u{202f}000\u{202f}000", buf.as_str());
/// }
/// ```
#[derive(Copy, Clone)]
pub struct Buffer {
    pub(crate) inner: [u8; MAX_BUF_LEN],
    pub(crate) pos: usize,
    pub(crate) end: usize,
}

impl Buffer {
    /// Constructs a new, stack-allocated buffer.
    #[inline(always)]
    pub fn new() -> Buffer {
        Buffer {
            inner: [0; MAX_BUF_LEN],
            pos: MAX_BUF_LEN,
            end: MAX_BUF_LEN,
        }
    }

    /// Returns a `&[u8]` view into the buffer.
    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        &self.inner[self.pos..self.end]
    }

    /// Returns a `&str` view into the buffer.
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }

    /// Returns `true` if the buffer is empty; `false` otherwise.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the length (in bytes) of the buffer.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.end - self.pos
    }

    /// Writes the provided number into the buffer using the provided format.
    #[inline(always)]
    pub fn write_formatted<F, N>(&mut self, n: &N, format: &F) -> usize
    where
        F: Format,
        N: ToFormattedStr,
    {
        n.read_to_buffer(self, format)
    }

    #[inline(always)]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut u8 {
        self.inner.as_mut_ptr()
    }

    #[inline(always)]
    pub(crate) fn write_with_itoa<N: itoa::Integer>(&mut self, n: N) -> usize {
        let mut itoa_buf = itoa::Buffer::new();

        let s = itoa_buf.format(n);
        let s_len = s.len();

        self.pos = MAX_BUF_LEN - s_len;
        self.end = MAX_BUF_LEN;

        let dst = &mut self.inner[self.pos..self.end];
        dst.copy_from_slice(s.as_bytes());

        s_len
    }
}

impl AsRef<str> for Buffer {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for Buffer {
    #[inline(always)]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for Buffer {
    /// Same as the [`new`] method.
    ///
    /// [`new`]: struct.Buffer.html#method.new
    #[inline(always)]
    fn default() -> Buffer {
        Buffer::new()
    }
}

impl Deref for Buffer {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(feature = "with-serde")]
mod serialization {
    use serde::{de, ser};

    use super::*;

    impl ser::Serialize for Buffer {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            serializer.serialize_bytes(self.as_bytes())
        }
    }

    impl<'de> de::Deserialize<'de> for Buffer {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct BufferVisitor;

            impl<'de> de::Visitor<'de> for BufferVisitor {
                type Value = Buffer;

                fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "bytes of maximum length {}", MAX_BUF_LEN)
                }

                fn visit_seq<V>(self, mut seq: V) -> Result<Buffer, V::Error>
                where
                    V: de::SeqAccess<'de>,
                {
                    let mut inner: [u8; MAX_BUF_LEN] = [0; MAX_BUF_LEN];
                    let mut index = 0;
                    while let Some(value) = seq.next_element()? {
                        if index < MAX_BUF_LEN {
                            inner[index] = value;
                            index += 1;
                        } else {
                            return Err(de::Error::invalid_length(index, &self));
                        }
                    }
                    Ok(Buffer {
                        inner,
                        pos: 0,
                        end: index,
                    })
                }
            }

            deserializer.deserialize_seq(BufferVisitor)
        }
    }

    #[cfg(test)]
    mod tests {
        use arrayvec::ArrayString;

        use crate::constants::MAX_BUF_LEN;
        use crate::{Buffer, Locale};

        #[test]
        fn test_buffer_serialization() {
            let mut buf = Buffer::new();
            let _ = buf.write_formatted(&1_000, &Locale::en);
            let s = serde_json::to_string(&buf).unwrap();
            assert_eq!(&s, "[49,44,48,48,48]");
        }

        #[test]
        fn test_buffer_deserialization() {
            // should pass
            let buf: Buffer = serde_json::from_str("[49,44,48,48,48]").unwrap();
            assert_eq!(0, buf.pos);
            assert_eq!(5, buf.end);
            assert_eq!(&[49, 44, 48, 48, 48], buf.as_bytes());
            assert_eq!("1,000", buf.as_str());

            // should fail
            let mut should_fail = ArrayString::<1024>::new();
            should_fail.push_str("[0");
            for _ in 0..MAX_BUF_LEN {
                should_fail.push_str(",0");
            }
            should_fail.push(']');
            let result: Result<Buffer, serde_json::Error> = serde_json::from_str(&should_fail);
            if result.is_ok() {
                panic!("was somehow able to deserialize bytes that were too long")
            }
        }

        #[test]
        fn test_buffer_bincode() {
            // Control constants.
            const TEST_STR1: &str = "1,000";
            const TEST_BUFFER1: &[u8] = TEST_STR1.as_bytes(); // [49,44,48,48,48]
            const TEST_STR2: &str = "10,000";
            const TEST_BUFFER2: &[u8] = TEST_STR2.as_bytes(); // [49,48,44,48,48,48]

            // Create a struct with some byte padding.
            #[derive(serde::Serialize, serde::Deserialize)]
            struct Struct {
                pad1: [u8; 1],
                buf1: Buffer,
                pad2: [u8; 3],
                buf2: Buffer,
                pad3: [u8; 5],
            }

            let mut s = Struct {
                pad1: [1_u8; 1],
                buf1: Buffer::new(),
                pad2: [0_u8; 3],
                buf2: Buffer::new(),
                pad3: [1_u8; 5],
            };

            // Write to buffers.
            s.buf1.write_formatted(&1_000, &Locale::en);
            s.buf2.write_formatted(&10_000, &Locale::en);

            // Serialize.
            let bytes: Vec<u8> = bincode::serialize(&s).unwrap();

            // Deserialize.
            let s: Struct = bincode::deserialize(&bytes).unwrap();

            // Assert the inner buffers made it through ok.
            assert_eq!(0, s.buf1.pos);
            assert_eq!(5, s.buf1.end);
            assert_eq!(0, s.buf2.pos);
            assert_eq!(6, s.buf2.end);
            assert_eq!(TEST_BUFFER1, s.buf1.as_bytes());
            assert_eq!(TEST_STR1, s.buf1.as_str());
            assert_eq!(TEST_BUFFER2, s.buf2.as_bytes());
            assert_eq!(TEST_STR2, s.buf2.as_str());
        }
    }
}
