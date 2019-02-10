#![cfg(feature = "std")]

use num_format::SystemLocale;

#[cfg(unix)]
#[test]
fn test_system_locale_unix() {
    use num_format::ToFormattedString;
    use std::collections::HashSet;
    use std::env;
    use std::process::Command;

    let n = -100_000isize;

    let names = SystemLocale::available_names().unwrap();

    for name in &names {
        let locale1 = SystemLocale::from_name(name.to_string()).unwrap();
        let s1 = n.to_formatted_string(&locale1);

        env::set_var("LC_ALL", name.to_string());
        let locale2 = SystemLocale::new().unwrap();
        let s2 = n.to_formatted_string(&locale2);

        assert_eq!(s1, s2);
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

    assert_eq!(names, from_command_line);
}

#[cfg(windows)]
#[test]
fn test_system_locale_windows() {
    let names = SystemLocale::available_names().unwrap();
    assert!(!names.is_empty());
    for name in &names {
        let _ = SystemLocale::from_name(name.as_ref()).unwrap();
    }
}
