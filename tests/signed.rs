#![feature(crate_visibility_modifier)]

mod common;

use num_display::DisplayInteger;

use self::common::*;

#[test]
fn test_signed() {
    let mut counts = [0, 0, 0, 0, 0];
    for (i, input) in INPUT.iter().enumerate() {
        if let Ok(input) = input.parse::<i8>() {
            assert_eq!(&input.to_formatted(), EXPECTED[i]);
            counts[0] += 1;
        }
        if let Ok(input) = input.parse::<i16>() {
            assert_eq!(&input.to_formatted(), EXPECTED[i]);
            counts[1] += 1;
        }
        if let Ok(input) = input.parse::<i32>() {
            assert_eq!(&input.to_formatted(), EXPECTED[i]);
            counts[2] += 1;
        }
        if let Ok(input) = input.parse::<i64>() {
            assert_eq!(&input.to_formatted(), EXPECTED[i]);
            counts[3] += 1;
        }
        if let Ok(input) = input.parse::<i128>() {
            assert_eq!(&input.to_formatted(), EXPECTED[i]);
            counts[4] += 1;
        }
        if let Ok(input) = input.parse::<isize>() {
            assert_eq!(&input.to_formatted(), EXPECTED[i]);
        }
    }
    assert_eq!(counts, [6, 12, 24, 46, 86]);
}
