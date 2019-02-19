# num-format-windows

A helper crate for [num-format] that only exists because num-format on Windows requires
[bindgen] as a build dependency, which, if not broken out into a separate crate, would
prevent num-format from being built in a `no_std` environment (build dependendies cannot
currently be made contingent on the compilation target).

Said differently, don't use this crate directly. It's just a work-around for
[https://github.com/rust-lang/rust-bindgen/issues/1439](https://github.com/rust-lang/rust-bindgen/issues/1439).

[bindgen]: https://github.com/rust-lang/rust-bindgen
[num-format]: https://github.com/bcmyers/num-format
