#![cfg(unix)]

use std::cmp::Ordering;
use std::collections::HashSet;
use std::env;
use std::process::Command;

use num_format_system::SystemLocale;

#[test]
fn test_unix() {
    let set = SystemLocale::available_names().unwrap();
    let mut vec = set.iter().map(|s| s.to_string()).collect::<Vec<String>>();
    vec.sort_by(|_, _| {
        if rand::random() {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });
    for name in &vec {
        let locale1 = SystemLocale::from_name(name.to_string()).unwrap();
        env::set_var("LC_ALL", name);
        let locale2 = SystemLocale::default().unwrap();
        assert_eq!(locale1, locale2);
    }

    let from_command_line = {
        let output = Command::new("locale").arg("-a").output().unwrap();
        if !output.status.success() {
            panic!()
        }
        let stdout = std::str::from_utf8(&output.stdout).unwrap();
        stdout
            .lines()
            .map(|s| s.trim().to_string())
            .collect::<HashSet<String>>()
    };
    assert_eq!(set, from_command_line);
}
