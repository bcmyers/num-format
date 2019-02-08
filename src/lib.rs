/*!
[![Build Status](https://travis-ci.org/bcmyers/num-format.svg?branch=master)](https://travis-ci.org/bcmyers/num-format)
[![Crates.io](https://img.shields.io/crates/v/num-format.svg)](https://crates.io/crates/num-format)
[![Documentation](https://docs.rs/num-format/badge.svg)](https://docs.rs/num-format/)
![License](https://img.shields.io/crates/l/num_format.svg)

A Rust crate for producing string-representations of numbers, formatted according to international standards,
e.g.
- `"1,000,000"` for US English
- `"10,00,000"` for Indian English
- `"1 000 000"` for French French

# Picking a format

Formatting options (e.g. which thousands separator to use, what the minus sign looks like, etc.) are represented by
the [`Format`] trait. This crate offers three concrete implementations of the [`Format`] trait...

### `Locale`

The [`Locale`] type is a programatically generated enum representing formatting standards from the
[Common Locale Data Repository], which is maintained by the [Unicode Consortium] and used by Apple in macOS and iOS,
by LibreOffice, by IBM in AIX, among others.

```rust
use num_format::{Format, Locale};

fn main() {
    let format = Locale::en;
    assert_eq!(format.decimal(), '.');;
    assert_eq!(format.minus_sign(), "-");
    assert_eq!(format.separator(), Some(','))
    // ...
}
```

### `SystemLocale`

The [`SystemLocale`] type allows you to access your system's locale settings via the `LC_ALL` environment variable.
If you're familiar with C, it pulls system information using the [`setlocale`] and [`localeconv`] functions in the C
standard library. For more details, see [`SystemLocale`].

### `CustomFormat`

Allows for the creation of your own, custom format. For more details, see [`CustomFormat`].

# Creating a string representation

Once you have selected a format, you can turn number types into formatted string representations via
any of three principle APIs...

### `ToFormattedString`

Using the [`ToFormattedString`] trait is the simplist API, just call [`to_formatted_string`] on a type that implements
it (all the number types in the standard library implement it) with a desired format. That said, using
[`ToFormattedString`] will always heap allocate; so it is the slowest of the three APIs and cannot be used in a
`no_std` environment.

```rust
# use cfg_if::cfg_if; cfg_if! { if #[cfg(feature = "std")] {
use num_format::{Locale, ToFormattedString};

fn main() {
    let s = 1000000.to_formatted_string(&Locale::en);
    assert_eq!(&s, "1,000,000");
}
# } else { fn main() {} } }
```

### `Buffer`

Using the [`Buffer`] type is the fastest API, as it does **not** heap allocate. Instead, the formatted representation
is written into a stack-allocated buffer. As such, you can use it in a `no_std` environment.

Although this API is available for all the number types in the standard library, it is **not** available
for third party types like [`num_bigint::BigInt`] since their maximum size cannot be known in advance.

```rust
use num_format::{Buffer, Locale};

fn main() {
    // Create a stack-allocated buffer...
    let mut buf = Buffer::default();

    // Write '"1,000,000"' into the buffer...
    buf.write_formatted(&1000000, &Locale::en);

    assert_eq!(buf.as_str(), "1,000,000");
}
```

### `WriteFormatted`

The [`WriteFormatted`] trait is in between the other two APIs. You can write a formatted representation into
any type that implements [`WriteFormatted`] (all the types in the standard library that implement [`io::Write`] or
[`fmt::Write`] implement it, such as [`Vec`], [`String`], [`File`], etc.).

If you're writing a number type that can use the [`Buffer`] API (e.g. any number type in the standard library), there
is **no** heap allocation. That said, you can also use this API with types where the [`Buffer`] API will not work, like
[`num_bigint::BigInt`], in which case there will be heap allocations used. This trait is **not** available
in a `no_std` environment.

```rust
# use cfg_if::cfg_if; cfg_if! { if #[cfg(feature = "std")] {
use num_format::{Locale, WriteFormatted};

fn main() {
    // Create a writer...
    let mut writer = String::new(); // Could also be Vec::new(), File::open(...), ...

    // Write '"1,000,000"' into the writer...
    writer.write_formatted(&1000000, &Locale::en);

    assert_eq!(&writer, "1,000,000");
}
# } else { fn main() {} } }
```

# Extra features

| Available features | What to put in your `Cargo.toml`                              |
| :----------------- | :------------------------------------------------------------ |
| `no_std`           | `num-format = { version = "0.1", default-features = false }`  |
| `num-bigint`       | `num-format = { version = "0.1", features = ["num-bigint"] }` |
| `serde`            | `num-format = { version = "0.1", features = ["with-serde"] }` |

# License

**num-format** is licensed under either of:

- [The Apache License, Version 2.0], or
- [The MIT license]

at your option.

[`Buffer`]: struct.Buffer.html
[Common Locale Data Repository]: https://en.wikipedia.org/wiki/Common_Locale_Data_Repository
[`CustomFormat`]: format/struct.CustomFormat.html
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`fmt::Write`]: https://doc.rust-lang.org/std/fmt/fn.write.html
[`Format`]: format/trait.Format.html
[`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
[`Locale`]: format/enum.Locale.html
[`localeconv`]: https://www.gnu.org/software/libc/manual/html_node/The-Lame-Way-to-Locale-Data.html#The-Lame-Way-to-Locale-Data
[`num_bigint::BigInt`]: https://docs.rs/num-bigint/0.2.2/num_bigint/struct.BigInt.html
[`setlocale`]: https://www.gnu.org/software/libc/manual/html_node/Setting-the-Locale.html
[`String`]: https://doc.rust-lang.org/std/string/struct.String.html
[`SystemLocale`]: format/struct.SystemLocale.html
[The Apache License, Version 2.0]: http://www.apache.org/licenses/LICENSE-2.0
[The MIT license]: http://opensource.org/licenses/MIT
[`ToFormattedString`]: trait.ToFormattedString.html
[`to_formatted_string`]: trait.ToFormattedString.html#method.to_formatted_string
[Unicode Consortium]: https://en.wikipedia.org/wiki/Unicode_Consortium
[`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[`WriteFormatted`]: trait.WriteFormatted.html
*/

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    dead_code,
    deprecated,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_imports,
    unused_macros,
    unused_mut,
    unused_results,
    unused_parens,
    unused_unsafe,
    unused_variables
)]
#![doc(html_root_url = "https://docs.rs/num-format/0.3.0")]

#[cfg(feature = "with-serde")]
#[macro_use]
extern crate serde;

mod buffer;
mod constants;
mod custom_format;
mod custom_format_builder;
mod error;
mod error_kind;
mod format;
mod grouping;
mod impls;
mod locale;
mod system_locale;
mod to_formatted_str;
mod to_formatted_string;
pub mod utils;
mod write_formatted;

pub use self::buffer::Buffer;
pub use self::custom_format::CustomFormat;
pub use self::custom_format_builder::CustomFormatBuilder;
pub use self::error::Error;
pub use self::error_kind::ErrorKind;
pub use self::format::Format;
pub use self::grouping::Grouping;
pub use self::locale::Locale;
#[cfg(feature = "std")]
pub use self::standard::*;
pub use self::to_formatted_str::ToFormattedStr;

#[cfg(feature = "std")]
mod standard {
    pub use super::system_locale::SystemLocale;
    pub use super::to_formatted_string::ToFormattedString;
    pub use super::write_formatted::WriteFormatted;
}

mod sealed {
    pub trait Sealed {}
}
