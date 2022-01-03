use std::path::PathBuf;

use crate::api::{context::SwoonContext, DEFAULT_OS, task, util::write_file};
use crate::api::template::{Template, template_object};
use crate::gcloud;

pub fn write_gcp_archetype_config(ctx: &SwoonContext, dest_dir: PathBuf) -> task::Result<()> {
    let image_name = "archetype";
    let source = render_gcp_source(ctx, "archetype")?;
    let build = render_build("archetype")?;
    let filename = format!("{}.pkr.hcl", image_name);
    let path = PathBuf::from(dest_dir).join(filename);
    write_file(&path, format!("{}\n\n{}", source, build))?;
    task::SUCCESS
}

fn render_build(image_name: &str) -> task::Result<String> {
    let source_name = format!("source.googlecompute.{}", image_name);
    let template = Template::new(include_bytes!("build.pkr.hcl.liquid"))?;
    Ok(template.render(&template_object!({
        "source_name": source_name,
        "image_name": image_name,
    }))?)
}

fn render_gcp_source(ctx: &SwoonContext, image_name: &str) -> task::Result<String> {
    let org_name = match &ctx.config {
        Some(cfg) => &cfg.org_name,
        None => return task::Error::result("no config"),
    };
    let gcp_proj_id = gcloud::cli::default_project_id(ctx)?;
    let source_image = gcloud::cli::image_name_by_os(ctx, DEFAULT_OS)?;
    let template = Template::new(include_bytes!("source.gcp.pkr.hcl.liquid"))?;
    Ok(template.render(&template_object!({
        "gcp_proj_id": gcp_proj_id,
        "org_name": org_name,
        "source_image": source_image,
        "image_name": image_name,
    }))?)
}
