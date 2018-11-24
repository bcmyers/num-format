use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::types::{Locale, Policy};

pub(crate) fn write_formats2<P>(out_dir: P, data: &Vec<(Locale, Policy)>)
where
    P: AsRef<Path>,
{
    let mut vec = Vec::new();
    for (locale, policy) in data.iter() {
        vec.push((
            locale.identifier.to_string(),
            policy.decimal_formats.to_string(),
        ));
    }
    let mut vec = vec
        .into_iter()
        .filter(|(_, ref format)| format != "#,##0.###")
        .collect::<Vec<(String, String)>>();
    vec.sort_by(|a, b| match a.1.cmp(&b.1) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => a.0.cmp(&b.0),
    });
    let mut buf = String::new();
    for (identifier, format) in vec.iter() {
        buf.push_str(identifier);
        buf.push_str(": ");
        buf.push_str(format);
        buf.push('\n');
    }
    let out_dir = out_dir.as_ref();
    let mut file = fs::File::create(out_dir).unwrap();
    file.write_all(buf.as_bytes()).unwrap();
}

pub(crate) fn write_formats<P>(out_dir: P, data: &Vec<(Locale, Policy)>)
where
    P: AsRef<Path>,
{
    let mut formats = HashSet::new();
    let mut separators = HashSet::new();
    let mut decimals = HashSet::new();
    let mut minus = HashSet::new();
    let mut infinity = HashSet::new();
    let mut nan = HashSet::new();
    for (_, policy) in data.iter() {
        formats.insert(policy.decimal_formats.to_string());
        separators.insert(policy.group.to_string());
        decimals.insert(policy.decimal.to_string());
        minus.insert(policy.minus_sign.to_string());
        infinity.insert(policy.infinity.to_string());
        nan.insert(policy.nan.to_string());
    }
    let mut buf = String::new();

    buf.push_str("formats\n");
    for x in formats.iter() {
        buf.push_str(x);
        buf.push('\n');
    }
    buf.push_str("\nseparators\n");
    for x in separators.iter() {
        buf.push_str(x);
        buf.push_str("\nDebug:\t");
        x.chars()
            .for_each(|c| c.escape_debug().for_each(|c| buf.push(c)));
        buf.push_str("\nUnicode:\t");
        x.chars()
            .for_each(|c| c.escape_unicode().for_each(|c| buf.push(c)));
        buf.push_str("\n\n");
    }
    buf.push_str("\ndecimal\n");
    for x in decimals.iter() {
        buf.push_str(x);
        buf.push_str("\nDebug:\t");
        x.chars()
            .for_each(|c| c.escape_debug().for_each(|c| buf.push(c)));
        buf.push_str("\nUnicode:\t");
        x.chars()
            .for_each(|c| c.escape_unicode().for_each(|c| buf.push(c)));
        buf.push_str("\n\n");
    }
    buf.push_str("\nminus\n");
    for x in minus.iter() {
        buf.push_str(x);
        buf.push_str("\nDebug:\t");
        x.chars()
            .for_each(|c| c.escape_debug().for_each(|c| buf.push(c)));
        buf.push_str("\nUnicode:\t");
        x.chars()
            .for_each(|c| c.escape_unicode().for_each(|c| buf.push(c)));
        buf.push_str("\n\n");
    }
    buf.push_str("\ninfinity\n");
    for x in infinity.iter() {
        buf.push_str(x);
        buf.push('\n');
    }
    buf.push_str("\nnan\n");
    for x in nan.iter() {
        buf.push_str(x);
        buf.push('\n');
    }

    let out_dir = out_dir.as_ref();
    let mut file = fs::File::create(out_dir).unwrap();
    file.write_all(buf.as_bytes()).unwrap();
}
