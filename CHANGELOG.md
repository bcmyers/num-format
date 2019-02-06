0.2.0 (unreleased)
==================
**Breaking changes:**

* Flatten module structure 
    * We decided to flatten the module structure of num-format to both improve code readabiity 
    and to simplify the importing of types by users. Unfortunately this is a breaking change.
    * The `num_format::format` and `num_format::errors` modules have been removed and all
    of their containing types have been moved to the crate root.
    * So whereas one used to do this:

    ```rust
    use num_format::errors::{Error, ErrorKind};
    use num_format::format::{CustomFormat, Environment, Format, Locale};
    use num_format::format::utils::{InfinityStr, MinusSignStr, NanStr};
    ```

    * one now does this:

    ```rust
    use num_format::{CustomFormat, Environment, Error, ErrorKind, Format, Locale};
    use num_format::utils::{InfinityStr, MinusSignStr, NanStr};
    ```

    * Thank you [BurntSushi](https://github.com/BurntSushi) for the [suggestion](https://github.com/bcmyers/num-format/issues/3#issuecomment-460615939)

* Refactor `Error` and `ErrorKind`
    * Since the future of the [failure crate](https://github.com/rust-lang-nursery/failure)
    is up in the air, we decided to remove it as a dependency, refactoring
    `num_format::Error` and `num_format::ErrorKind` to work with just plain old `std::error::Error`
    * Thank you [BurntSushi](https://github.com/BurntSushi) for the [suggestion](https://github.com/bcmyers/num-format/issues/3#issuecomment-460615939)

**New features:**

* impl `From<Environment>` and `From<Locale>` for `CustomFormat`
    * Now you can easiliy turn an `Environment` or a `Locale` into a `CustomFormat`; so it's easy
    to start with an already complete "base" format and then just tweak it slightly to the custom
    format you want.

    ```rust
    use num_format::{CustomFormat, Error, Locale, ToFormattedString};

    fn main() -> Result<(), Error> {
        let format = CustomFormat::from(Locale::en_IN)
            .into_builder()
            .separator(Some(' '))
            .build()?;
        let s = 1_000_000.to_formatted_string(&format);
        assert_eq!(&s, "10 00 000");
        Ok(())
    }
    ```

0.1.2 (2019-02-04)
==================
**Initial release**