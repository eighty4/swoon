use std::fs;

use crate::api::output::file::Directory;
use crate::api::output::file::Directory::{AnsibleRole, ProjectRoot};
use crate::api::task;

pub fn init_archetype_playbook() -> task::Result<()> {
    ProjectRoot.write_bytes(
        None,
        "archetype.yml",
        include_bytes!("archetype.yml").to_vec(),
    )
}

pub fn init_roles_dir() -> task::Result<()> {
    AnsibleRole { role_name: String::from("firewall") }.write_bytes(
        Some("tasks"),
        "main.yml",
        include_bytes!("roles/firewall.yml").to_vec()
    )
}

pub fn copy_archetype_playbook(dir: Directory) -> task::Result<()> {
    fs::copy("archetype.yml", dir.join_path("archetype.yml"))?;
    Ok(())
}
