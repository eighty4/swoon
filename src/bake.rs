use crate::{ansible, gcloud, packer};
use crate::api::{swoon_error_result, SwoonError};
use crate::api::context::SwoonContext;

pub struct BakeOpts {
    pub approve_plan: bool,
}

pub fn bake_machine_images(ctx: &SwoonContext, opts: &BakeOpts) -> Result<(), SwoonError> {
    if ctx.config_path == None {
        return swoon_error_result("swoon.yml already exists in the current dir");
    }

    let image_names = gcloud::cli::all_image_names(&ctx).unwrap();
    if image_names.is_empty() {
        println!("bake archetype machine image");
    } else {
        println!("maybe bake archetype machine image if existing archetype expired");
    }

    if opts.approve_plan {} else {}

    packer::write_gcp_archetype_config(ctx)?;
    ansible::write_archetype_playbook()?;

    return Result::Ok(());
}
