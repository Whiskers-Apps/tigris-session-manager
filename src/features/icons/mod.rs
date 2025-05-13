use std::path::PathBuf;

use tigris_core::features::extensions::get_extension_dir;

use crate::EXTENSION_ID;

pub fn get_icon(name: &str) -> PathBuf {
    get_extension_dir(EXTENSION_ID)
        .unwrap()
        .join(format!("src/features/icons/{name}.svg"))
}
