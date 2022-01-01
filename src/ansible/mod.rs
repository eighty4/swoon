use std::fs;
use std::io::Write;

use crate::api::task;

pub fn write_archetype_playbook() -> task::Result<()> {
    let mut file = fs::File::create("./archetype.yml")?;
    file.write_all(include_bytes!("archetype.yml"))?;
    task::SUCCESS
}
