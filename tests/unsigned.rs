#![feature(crate_visibility_modifier)]

mod common;

use num_display::DisplayInteger;

use self::common::*;

#[test]
fn test_unsigned() {
    let mut counts = [0, 0, 0, 0, 0];
    for (i, input) in INPUT.iter().enumerate() {
        if let Ok(input) = input.parse::<u8>() {
            assert_eq!(&input.to_formatted(), EXPECTED[i]);
            counts[0] += 1;
        }
        if let Ok(input) = input.parse::<u16>() {
            assert_eq!(&input.to_formatted(), EXPECTED[i]);
            counts[1] += 1;
        }
        if let Ok(input) = input.parse::<u32>() {
            assert_eq!(&input.to_formatted(), EXPECTED[i]);
            counts[2] += 1;
        }
        if let Ok(input) = input.parse::<u64>() {
            assert_eq!(&input.to_formatted(), EXPECTED[i]);
            counts[3] += 1;
        }
        if let Ok(input) = input.parse::<u128>() {
            assert_eq!(&input.to_formatted(), EXPECTED[i]);
            counts[4] += 1;
        }
        if let Ok(input) = input.parse::<usize>() {
            assert_eq!(&input.to_formatted(), EXPECTED[i]);
        }
    }
    assert_eq!(counts, [4, 7, 13, 24, 44]);
}
