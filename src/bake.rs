use crate::api::command;
use crate::api::command::Name::Init;
use crate::api::context::SwoonContext;
use crate::gcloud;

pub struct BakeOpts {
    pub approve_plan: bool,
}

pub fn bake_machine_images(ctx: &SwoonContext, opts: &BakeOpts) -> command::Result {
    if ctx.config_path == None {
        return command::Error::with_command_suggestions(
            "There's no swoon.yml file in your current directory",
            vec!(Init),
        );
    }

    let image_names = gcloud::cli::all_image_names(&ctx).unwrap();
    if image_names.is_empty() {
        println!("Bake archetype machine image - no existing archetype image");
    } else {
        println!("{}", format!("Maybe bake archetype machine image - existing images: {}", image_names.join(",")));
    }

    if opts.approve_plan {} else {}

    command::SUCCESS
}
