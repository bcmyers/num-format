use std::ffi;
use std::fs;
use std::path::Path;

use indexmap::IndexMap;
use serde::Deserialize;
use serde_json;
use walkdir::WalkDir;

use crate::utils::{Format, Grouping};

const MAX_MIN_LEN: usize = 8;
const MAX_POS_LEN: usize = 8;

/// Walks a directory containing CLDR json files and collects the data they contain into a map.
pub fn parse_data<P>(data_dir: P) -> Result<IndexMap<String, Format>, failure::Error>
where
    P: AsRef<Path>,
{
    let mut data: IndexMap<String, Format> = IndexMap::new();

    // Walk the data dir
    for entry in WalkDir::new(data_dir.as_ref()) {
        let entry = entry.unwrap();

        // Skip if we aren't dealing with a "numbers.json" file
        if !entry.file_type().is_file() || entry.file_name() != ffi::OsStr::new("numbers.json") {
            continue;
        }

        // Read file
        let contents = fs::read_to_string(entry.path()).unwrap();

        // Get the variant name and identifier
        let identifier = entry
            .path()
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        let variant_name = make_variant_name(identifier);

        // Deserialize the json
        let value: serde_json::Value = serde_json::from_str(&contents).unwrap();
        let value = &value["main"][identifier];

        // Get the symbols
        let default_numbering_system =
            &value["numbers"]["defaultNumberingSystem"].as_str().unwrap();
        let symbols_lookup = format!("symbols-numberSystem-{}", default_numbering_system);
        let symbols = &value["numbers"][&symbols_lookup].to_string();
        let symbols: Symbols = serde_json::from_str(&symbols).unwrap();

        // Grouping
        let decimal_formats_lookup =
            format!("decimalFormats-numberSystem-{}", default_numbering_system);
        let decimal_formats = &value["numbers"][&decimal_formats_lookup];
        let grp = decimal_formats["standard"]
            .as_str()
            .unwrap()
            .parse::<Grouping>()
            .unwrap();

        // Format
        let format = Format {
            identifier: identifier.to_string(),

            dec: symbols.decimal,
            grp,
            inf: symbols.infinity,
            min: {
                let s = symbols.minus_sign.to_string();
                assert!(s.len() <= MAX_MIN_LEN);
                s
            },
            nan: symbols.nan,
            pos: {
                let s = symbols.plus_sign.to_string();
                assert!(s.len() <= MAX_POS_LEN);
                s
            },
            sep: symbols.group,
        };

        let _ = data.insert(variant_name, format);
    }
    data.sort_by(|k1, _, k2, _| k1.cmp(k2));
    Ok(data)
}

fn make_variant_name(identifier: &str) -> String {
    let mut buf = String::new();

    // Trim whitespace
    let s = identifier.trim();

    // Push characters into buffer, but replace '-' with '_'
    for c in s.chars() {
        if c == '-' {
            buf.push('_')
        } else {
            buf.push(c)
        }
    }

    // Special case "as" becuase it's is a keyword in Rust
    if &buf == "as" {
        buf.push('_');
    }

    buf
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Symbols {
    decimal: char,
    exponential: String,
    group: char,
    infinity: String,
    minus_sign: String,
    nan: String,
    percent_sign: String,
    plus_sign: String,
}
