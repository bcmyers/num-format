fn main() {
    run()
}

#[cfg(windows)]
fn run() {
    use std::env;
    use std::path::Path;

    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let headers_path = Path::new(&root).join("src").join("build").join("wrapper.h");
    let headers = headers_path.to_str().unwrap();

    let bindings = bindgen::Builder::default()
        .header(headers)
        .whitelist_var("LOCALE_NAME_SYSTEM_DEFAULT")
        .whitelist_var("LOCALE_SDECIMAL")
        .whitelist_var("LOCALE_SGROUPING")
        .whitelist_var("LOCALE_SMONDECIMALSEP")
        .whitelist_var("LOCALE_SMONGROUPING")
        .whitelist_var("LOCALE_SMONTHOUSANDSEP")
        .whitelist_var("LOCALE_SNEGATIVESIGN")
        .whitelist_var("LOCALE_STHOUSAND")
        .generate()
        .expect("unable to generate bindings for windows.h");

    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(&out_path)
        .expect("unable to write bindings for windows.h");
}

#[cfg(not(windows))]
fn run() {}
