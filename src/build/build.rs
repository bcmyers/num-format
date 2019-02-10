use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(all(feature = "std", windows))] {
        fn main() {
            use std::env;
            use std::path::Path;

            use bindgen::{Builder, RustTarget};

            let root = env::var("CARGO_MANIFEST_DIR").unwrap();
            let headers_path = Path::new(&root).join("src").join("build").join("win.h");
            let headers = headers_path.to_str().unwrap();

            let bindings = Builder::default()
                .header(headers)
                .rust_target(RustTarget::Stable_1_0)
                .whitelist_var("LOCALE_NAME_MAX_LENGTH")
                .whitelist_var("LOCALE_NAME_SYSTEM_DEFAULT")
                .whitelist_var("LOCALE_SDECIMAL")
                .whitelist_var("LOCALE_SGROUPING")
                .whitelist_var("LOCALE_SPOSINFINITY")
                .whitelist_var("LOCALE_SNAME")
                .whitelist_var("LOCALE_SNAN")
                .whitelist_var("LOCALE_SNEGATIVESIGN")
                .whitelist_var("LOCALE_SNEGINFINITY")
                .whitelist_var("LOCALE_STHOUSAND")
                .generate()
                .expect("unable to generate bindings for windows.h");

            let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("windows.rs");
            bindings
                .write_to_file(&out_path)
                .expect("unable to write bindings for windows.h");

            let development_dir = Path::new(&root).join("bindings");
            if development_dir.exists() {
                let out_path = development_dir.join("windows.rs");
                bindings
                    .write_to_file(&out_path)
                    .expect("unable to write bindings for windows.h");
            }
        }
    } else if #[cfg(all(
        feature = "std",
        any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "openbsd",
            target_os = "netbsd"
        )
    ))] {
        fn main() {
            use std::env;
            use std::path::Path;

            use bindgen::{Builder, RustTarget};

            let root = env::var("CARGO_MANIFEST_DIR").unwrap();
            let headers_path = Path::new(&root).join("src").join("build").join("unix.h");
            let headers = headers_path.to_str().unwrap();

            let bindings = Builder::default()
                .header(headers)
                .rust_target(RustTarget::Stable_1_0)
                .whitelist_function("freelocale")
                .whitelist_function("newlocale")
                .whitelist_function("querylocale")
                .whitelist_function("uselocale")
                .whitelist_var("LC_MONETARY_MASK")
                .whitelist_var("LC_NUMERIC_MASK")
                .generate()
                .expect("unable to generate bindings for xlocale.h");

            let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("unix.rs");
            bindings
                .write_to_file(&out_path)
                .expect("unable to write bindings for xlocale.h");

            let development_dir = Path::new(&root).join("bindings");
            if development_dir.exists() {
                let out_path = development_dir.join("unix.rs");
                bindings
                    .write_to_file(&out_path)
                    .expect("unable to write bindings for xlocale.h");
            }
        }
    } else #[cfg(all(feature = "std", unix))] {
        fn main() {
            // use std::env;
            // use std::path::Path;

            // use bindgen::{Builder, RustTarget};

            // let root = env::var("CARGO_MANIFEST_DIR").unwrap();
            // let headers_path = Path::new(&root).join("src").join("build").join("nix.h");
            // let headers = headers_path.to_str().unwrap();

            // let bindings = Builder::default()
            //     .header(headers)
            //     .rust_target(RustTarget::Stable_1_0)
            //     .whitelist_function("freelocale")
            //     .whitelist_function("newlocale")
            //     .whitelist_function("uselocale")
            //     .whitelist_var("LC_MONETARY_MASK")
            //     .whitelist_var("LC_NUMERIC_MASK")
            //     .generate()
            //     .expect("unable to generate bindings for locale.h");

            // let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("nix.rs");
            // bindings
            //     .write_to_file(&out_path)
            //     .expect("unable to write bindings for locale.h");

            // let development_dir = Path::new(&root).join("bindings");
            // if development_dir.exists() {
            //     let out_path = development_dir.join("nix.rs");
            //     bindings
            //         .write_to_file(&out_path)
            //         .expect("unable to write bindings for locale.h");
            // }
        }
    } else {
        fn main() {}
    }
}
