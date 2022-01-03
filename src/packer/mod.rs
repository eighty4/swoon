use std::fs;
use std::path::PathBuf;

use crate::api::context::{BinaryPaths, SwoonContext};
use crate::api::process::Process;
use crate::api::task;
use crate::api::util::DataDir;
use crate::packer::config::write_gcp_archetype_config;

pub mod config;

pub fn bake_archetype_image(ctx: &SwoonContext) -> task::Result<()> {
    let org_name = match &ctx.config {
        Some(cfg) => &cfg.org_name,
        None => return task::Error::result("no config"),
    };
    let image_name = "archetype";
    let dest_dir = DataDir::create_sub_dir(
        format!("images/{}/{}", org_name, image_name).as_str())?;
    fs::copy("./archetype.yml", dest_dir.join("archetype.yml"))?;
    write_gcp_archetype_config(ctx, PathBuf::from(&dest_dir))?;
    Process::invoke_from_dir(dest_dir, ctx.gcloud_bin_path(), ["build"])?;
    task::SUCCESS
}
