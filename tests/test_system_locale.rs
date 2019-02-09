#![cfg(feature = "std")]

#[cfg(unix)]
#[test]
fn test_system_locale_unix() {
    use std::env;
    use num_format::{ToFormattedString, SystemLocale};

    let n = -100_000isize;

    let names = SystemLocale::available_names().unwrap();

    for name in &names {
        let locale1 = SystemLocale::from_name(name).unwrap();
        let s1 = n.to_formatted_string(&locale1);

        env::set_var("LC_ALL", name);
        let locale2 = SystemLocale::new().unwrap();
        let s2 = n.to_formatted_string(&locale2);

        assert_eq!(s1, s2);
    }
}
