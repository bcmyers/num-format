#[test]
fn test_errors_capacity1() {
    use num_format::utils::SeparatorStr;

    let s = "123456789";
    match SeparatorStr::new(s) {
        Ok(_) => panic!(),
        Err(e) => assert_eq!(
            "Attempted to write input of length 9 bytes into a buffer with capacity 8 bytes.",
            &e.to_string(),
        ),
    }
}

#[test]
fn test_errors_capacity2() {
    use num_format::CustomFormat;

    let s = "123456789";
    match CustomFormat::builder().separator(s).build() {
        Ok(_) => panic!(),
        Err(e) => assert_eq!(
            "Attempted to write input of length 9 bytes into a buffer with capacity 8 bytes.",
            &e.to_string(),
        ),
    }
}

#[test]
fn test_errors_parse_locale1() {
    use num_format::Locale;

    let s = "123456789";
    match Locale::from_name(s) {
        Ok(_) => panic!(),
        Err(e) => assert_eq!(
            "Failed to parse 123456789 into a valid locale.",
            &e.to_string(),
        ),
    }
}

#[cfg(all(feature = "std", any(unix, windows)))]
#[test]
fn test_errors_parse_locale2() {
    use num_format::SystemLocale;

    let s = "123456789";
    match SystemLocale::from_name(s) {
        Ok(_) => panic!(),
        Err(e) => assert_eq!(
            "Failed to parse 123456789 into a valid locale.",
            &e.to_string(),
        ),
    }
}

#[cfg(all(feature = "std", any(unix, windows)))]
#[test]
fn test_errors_interior_null_byte() {
    use std::str;

    use num_format::SystemLocale;

    let b = b"Hello\0World";
    let s = str::from_utf8(&b[..]).unwrap();
    match SystemLocale::from_name(s) {
        Ok(_) => panic!(),
        Err(e) => assert_eq!(
            "Locale name Hello\u{0}World contains an interior nul byte, which is not allowed.",
            &e.to_string()
        ),
    }
}
