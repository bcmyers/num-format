use std::ffi;
use std::fs;
use std::io::Read;
use std::path::Path;

use serde::Deserialize;
use serde_json;
use walkdir::WalkDir;

use crate::types::{Grouping, Locale, Policy};

#[derive(Debug, Eq, PartialEq, Deserialize)]
struct PartialLocale {
    language: String,
    script: Option<String>,
    territory: Option<String>,
    variant: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Symbols {
    decimal: String,
    group: String,
    infinity: String,
    minus_sign: String,
    nan: String,
}

pub(crate) fn parse_data<P>(data_dir: P) -> Vec<(Locale, Policy)>
where
    P: AsRef<Path>,
{
    let mut data = Vec::new();
    for entry in WalkDir::new(data_dir.as_ref()) {
        let entry = entry.unwrap();
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.file_name() != ffi::OsStr::new("numbers.json") {
            continue;
        }

        // Read file
        let mut file = fs::File::open(entry.path()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Deserialize json
        let value: serde_json::Value = serde_json::from_str(&contents).unwrap();
        let identifier = entry
            .path()
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let value = &value["main"][&identifier];

        // Locale
        let partial = &value["identity"].to_string();
        let partial: PartialLocale = serde_json::from_str(&partial).unwrap();
        let identifier = match identifier.as_ref() {
            "as" => "as_".to_string(),
            _ => identifier,
        };
        let language = match partial.language.as_ref() {
            "as" => "as_".to_string(),
            _ => partial.language.clone(),
        };
        let locale = Locale {
            identifier,
            language,
            script: partial.script.clone(),
            territory: partial.territory.clone(),
            variant: partial.variant.clone(),
        };

        // Symbols
        let default_numbering_system =
            &value["numbers"]["defaultNumberingSystem"].as_str().unwrap();
        let symbols_lookup = format!("symbols-numberSystem-{}", default_numbering_system);
        let symbols = &value["numbers"][&symbols_lookup].to_string();
        let symbols: Symbols = serde_json::from_str(&symbols).unwrap();

        // Decimal formats
        let decimal_formats_lookup =
            format!("decimalFormats-numberSystem-{}", default_numbering_system);
        let decimal_formats = &value["numbers"][&decimal_formats_lookup];
        let decimal_formats = decimal_formats["standard"]
            .as_str()
            .unwrap()
            .parse::<Grouping>()
            .unwrap();

        // Policy
        let policy = Policy {
            decimal: symbols.decimal.clone(),
            decimal_formats,
            group: symbols.group.clone(),
            infinity: symbols.infinity.clone(),
            minus_sign: symbols.minus_sign.clone(),
            nan: symbols.nan.clone(),
        };

        data.push((locale, policy));
    }
    data
}
