use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

const DIF_STATIC: &'static str = "#[no_mangle]\npub static DIF_FILE: &'static [(&'static str, &'static str)] = &";

/// Creates a Rust file called ``dif.rs`` which has the DIF turned into a Rust static which can be
/// turned into a [``Dif``](todo) value.
pub fn add_dif(dif_path: &str) {
    let dif = File::open(dif_path);
    let rust_dif = File::create("src/dif.rs"); // Should be included in lib.rs for device driver

    if dif.is_err() {
        panic!("Failed to open DIF file ({})", dif_path);
    } else if rust_dif.is_err() {
        panic!("Failed to create Rust DIF file (src/dif.rs)");
    }

    dif.unwrap().write_fmt(format_args!("{}{}{}", DIF_STATIC, read_to_string(Path::new(dif_path)).unwrap(), ";\n"));
}
