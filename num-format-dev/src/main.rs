use std::fs::File;
use std::io::Write;

use num_format_dev::{create_module, parse_data};

const DATA_DIR: &str = "./num-format-dev/cldr-numbers-full";
const OUT_PATH: &str = "./num-format/src/locale.rs";

fn main() -> Result<(), failure::Error> {
    let data = parse_data(DATA_DIR)?;
    let s = create_module(&data)?;
    let mut f = File::create(OUT_PATH)?;
    f.write_all(s.as_bytes())?;
    Ok(())
}
