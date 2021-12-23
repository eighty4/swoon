use std::fs;
use std::io::Write;

use crate::{swoon_error_result, SwoonContext, SwoonError};
use crate::api::template::{Template, template_object};
use crate::gcloud::cli;
use crate::gcloud::cli::GcpImageFamily;

pub fn write_gcp_archetype_config(ctx: &SwoonContext) -> Result<(), SwoonError> {
    let template = Template::new(include_bytes!("gcp.pkr.hcl.liquid"))?;
    let org_name = match &ctx.config {
        Some(cfg) => &cfg.org_name,
        None => return swoon_error_result("no config"),
    };
    let gcp_proj_id = cli::default_project_id(ctx)?;
    let source_image = cli::image_name_by_family(ctx, GcpImageFamily::Debian { version: 11 })?;
    let foo = template.render(&template_object!({
        "gcp_proj_id": gcp_proj_id,
        "org_name": org_name,
        "source_image": source_image,
    }))?;
    fs::create_dir_all("./.swoon")?;
    let mut file = fs::File::create("./.swoon/archetype.pkr.hcl")?;
    file.write_all(foo.as_bytes())?;
    return Ok(());
}
