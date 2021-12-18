use crate::api::{swoon_error_result, SwoonError};
use crate::api::context::SwoonContext;

pub struct BakeOpts {
    pub approve_plan: bool,
}

pub fn bake_machine_images(ctx: &SwoonContext, opts: &BakeOpts) -> Result<(), SwoonError> {
    if ctx.config_path == None {
        return swoon_error_result("swoon.yml already exists in the current dir");
    }
    // todo generate and print machine image plan
    if opts.approve_plan {
        println!("gen baking plan and bake resolved machine images");
    } else {
        println!("gen and print baking plan for approval, bake upon approval");
    }
    return Result::Ok(());
}
