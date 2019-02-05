use core::borrow::Borrow;
use core::fmt;
use core::mem;
use core::ops::Deref;
use core::str;

use crate::constants::MAX_BUF_LEN;
use crate::format::Format;
use crate::traits::ToFormattedStr;

/// A stack-allocated buffer that you can use to get a formatted `&str`
/// without heap allocation.
///
/// # Example
/// ```
/// use num_format::{Buffer, format::Locale};
///
/// fn main() {
///     // Create a stack-allocated buffer...
///     let mut buf = Buffer::default();
///
///     // Write '"1,000,000"' into the buffer...
///     buf.write_formatted(&1000000, &Locale::en);
///
///     // Get a view into the buffer as a `&str`...
///     let s = buf.as_str();
///
///     // Do what you want with the `&str`...
///     assert_eq!("1,000,000", s);
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
            inner: unsafe { mem::uninitialized() },
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
}

impl AsRef<str> for Buffer {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for Buffer {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

                fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "bytes of maximum length {}", MAX_BUF_LEN)
                }

                fn visit_seq<V>(self, mut seq: V) -> Result<Buffer, V::Error>
                where
                    V: de::SeqAccess<'de>,
                {
                    let mut inner: [u8; MAX_BUF_LEN] = unsafe { mem::uninitialized() };
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

            deserializer.deserialize_bytes(BufferVisitor)
        }
    }
}
