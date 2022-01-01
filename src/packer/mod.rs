use std::fs;
use std::io::Write;

use crate::{gcloud, SwoonContext};
use crate::api::{DEFAULT_OS, task};
use crate::api::template::{Template, template_object};

pub fn write_gcp_archetype_config(ctx: &SwoonContext) -> task::Result<()> {
    let template = Template::new(include_bytes!("gcp.pkr.hcl.liquid"))?;
    let org_name = match &ctx.config {
        Some(cfg) => &cfg.org_name,
        None => return task::Error::result("no config"),
    };
    let gcp_proj_id = gcloud::cli::default_project_id(ctx)?;
    let source_image = gcloud::cli::image_name_by_os(ctx, DEFAULT_OS)?;
    let foo = template.render(&template_object!({
        "gcp_proj_id": gcp_proj_id,
        "org_name": org_name,
        "source_image": source_image,
    }))?;
    fs::create_dir_all("./.swoon")?;
    let mut file = fs::File::create("./.swoon/archetype.pkr.hcl")?;
    file.write_all(foo.as_bytes())?;
    task::SUCCESS
}
