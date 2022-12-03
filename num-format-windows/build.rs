#[cfg(windows)]
fn main() {
    use std::env;
    use std::path::Path;

    use bindgen::{Builder, RustTarget};

    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let headers_path = Path::new(&root).join("wrapper.h");
    let headers = headers_path.to_str().unwrap();

    let bindings = Builder::default()
        .header(headers)
        .rust_target(RustTarget::Stable_1_33)
        .allowlist_var("LOCALE_NAME_MAX_LENGTH")
        .allowlist_var("LOCALE_NAME_SYSTEM_DEFAULT")
        .allowlist_var("LOCALE_SDECIMAL")
        .allowlist_var("LOCALE_SGROUPING")
        .allowlist_var("LOCALE_SNAME")
        .allowlist_var("LOCALE_SNAN")
        .allowlist_var("LOCALE_SNEGATIVESIGN")
        .allowlist_var("LOCALE_SNEGINFINITY")
        .allowlist_var("LOCALE_SPOSINFINITY")
        .allowlist_var("LOCALE_SPOSITIVESIGN")
        .allowlist_var("LOCALE_STHOUSAND")
        .generate()
        .expect("unable to generate bindings for windows.h");

    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(&out_path)
        .expect("unable to write bindings for windows.h");
}

#[cfg(not(windows))]
fn main() {}
