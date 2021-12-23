use std::fs;
use std::io::Write;

use crate::SwoonError;

pub fn write_archetype_playbook() -> Result<(), SwoonError> {
    let mut file = fs::File::create("./archetype.yml")?;
    file.write_all(include_bytes!("archetype.yml"))?;
    return Ok(());
}
