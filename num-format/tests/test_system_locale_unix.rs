#![cfg(all(feature = "std", unix))]

use std::cmp::Ordering;
use std::env;

use num_format::SystemLocale;

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
    assert!(!vec.is_empty());
    for name in &vec {
        let locale1 = SystemLocale::from_name(name.to_string()).unwrap();
        println!("{:?}", &locale1);
        env::set_var("LC_ALL", name);
        let locale2 = SystemLocale::default().unwrap();
        assert_eq!(locale1, locale2);
    }
}
