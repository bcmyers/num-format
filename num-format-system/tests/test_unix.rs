#![cfg(unix)]

use std::env;
use std::process::Command;

use num_format_system::SystemLocale;

#[test]
fn test_unix() {
    let names = SystemLocale::available_names().unwrap();
    let mut names = names.into_iter().collect::<Vec<String>>();
    names.sort();
    for name in &names {
        let locale1 = SystemLocale::from_name(name.to_string()).unwrap();
        env::set_var("LC_ALL", name);
        let locale2 = SystemLocale::default().unwrap();
        assert_eq!(locale1, locale2);
    }

    let mut from_command_line = {
        let output = Command::new("locale").arg("-a").output().unwrap();
        if !output.status.success() {
            panic!()
        }
        let stdout = std::str::from_utf8(&output.stdout).unwrap();
        stdout
            .lines()
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>()
    };
    from_command_line.sort();
    assert_eq!(names, from_command_line);
}
