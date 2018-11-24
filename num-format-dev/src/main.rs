mod parse_data;
mod types;
mod write_file;
mod write_formats;

use crate::parse_data::parse_data;
use crate::write_file::write_file;
use crate::write_formats::{write_formats, write_formats2};

fn main() {
    let data = parse_data("./cldr-numbers-full/main");
    write_formats("./temp1.txt", &data);
    write_formats2("./temp2.txt", &data);
    write_file("../src/locale2.rs", data);
}
