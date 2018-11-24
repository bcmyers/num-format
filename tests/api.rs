#![feature(crate_visibility_modifier)]

use num_display::details::Grouping;
use num_display::{Custom, DisplayInteger, Locale};

#[test]
fn test_api() {
    println!("{}", std::f64::NAN);
    println!("{}", std::f64::INFINITY);
    println!("{}", std::f64::NEG_INFINITY);

    let x = 1_000_000;

    // Format the number with the default locale, which is Locale::en_US
    let s1 = x.to_formatted();

    // Format the number with a different locale, e.g. fr_FR
    let s2 = x.to_formatted_with(&Locale::fr_FR);

    // Format the number with a custom format
    let my_format = Custom::builder()
        .decimal(' ')
        .grouping(Grouping::Indian)
        .minus_sign('-')
        .separator(' ')
        .build();
    let s3 = x.to_formatted_with(&my_format);

    // Format the number with a custom format built on the fly
    let s4 = x
        .formatter()
        .decimal('.')
        .grouping(Grouping::Standard)
        .minus_sign('-')
        .separator(',')
        .to_string();

    println!("{}", &s1);
    println!("{}", &s2);
    println!("{}", &s3);
    println!("{}", &s4);
}
