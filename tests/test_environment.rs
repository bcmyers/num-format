#![cfg(feature = "std")]
#![cfg(unix)]

use std::env;

use num_format::{Buffer, Environment};
use walkdir::WalkDir;

// TODO: Multi-platform support
const LOCALE_DIR: &str = "/usr/share/locale";

#[test]
fn test_environment() {
    let mut file_names = WalkDir::new(LOCALE_DIR)
        .max_depth(1)
        .into_iter()
        .map(|entry| {
            let entry = entry.unwrap();
            entry.file_name().to_str().unwrap().to_string()
        })
        .filter(|s| s != "locale")
        .collect::<Vec<String>>();
    file_names.sort_by(|a, b| a.cmp(b));
    for file_name in file_names {
        env::set_var("LC_ALL", &file_name);
        let environment = Environment::new().unwrap();
        let mut buf = Buffer::new();
        buf.write_formatted(&(-100_000), &environment);
    }
}
