#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(windows)] {
fn main() {
    use std::env;
    use std::path::Path;

    use bindgen::{Builder, RustTarget};

    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let headers_path = Path::new(&root).join("win.h");
    let headers = headers_path.to_str().unwrap();

    let bindings = Builder::default()
        .header(headers)
        .rust_target(RustTarget::Stable_1_28)
        .whitelist_var("LOCALE_NAME_MAX_LENGTH")
        .whitelist_var("LOCALE_NAME_SYSTEM_DEFAULT")
        .whitelist_var("LOCALE_SDECIMAL")
        .whitelist_var("LOCALE_SGROUPING")
        .whitelist_var("LOCALE_SNAME")
        .whitelist_var("LOCALE_SNAN")
        .whitelist_var("LOCALE_SNEGATIVESIGN")
        .whitelist_var("LOCALE_SNEGINFINITY")
        .whitelist_var("LOCALE_SPOSINFINITY")
        .whitelist_var("LOCALE_SPOSITIVESIGN")
        .whitelist_var("LOCALE_STHOUSAND")
        .generate()
        .expect("unable to generate bindings for windows.h");

    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("windows.rs");
    bindings
        .write_to_file(&out_path)
        .expect("unable to write bindings for windows.h");

    let development_dir = Path::new(&root).parent().unwrap().join("tmp");
    if development_dir.exists() {
        let out_path = development_dir.join("windows.rs");
        bindings
            .write_to_file(&out_path)
            .expect("unable to write bindings for windows.h");
    }
}
    } else {
        fn main() {}
    }
}
