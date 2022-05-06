use dialoguer::Input;

use crate::api::{command, task};
use crate::api::command::Name::Init;
use crate::api::context::SwoonContext;
use crate::api::util::DataDir;
use crate::images::BakingPlan;
use crate::packer::PackerBuild;

pub struct BakeOpts {
    pub approve_plan: bool,
}

pub fn bake_machine_images(ctx: &SwoonContext, opts: &BakeOpts) -> command::Result {
    if !ctx.has_config() {
        return command::Error::with_command_suggestions(
            "There's no swoon.yml file in your current directory",
            vec!(Init),
        );
    }
    DataDir::init()?;

    let baking_plan = BakingPlan::from(ctx)?;

    if !opts.approve_plan {
        if !prompt_for_approval(ctx, &baking_plan) {
            ctx.write_line("Cancelling bake plan");
            return command::SUCCESS;
        }
    }

    PackerBuild::default_archetype(ctx).bake()?;

    command::SUCCESS
}

fn prompt_for_approval(ctx: &SwoonContext, baking_plan: &BakingPlan) -> bool {
    let archetype_desc = format!("for {} on {}",
                                 baking_plan.default_archetype.source_string(),
                                 baking_plan.default_archetype.platform.to_str());
    ctx.write_line(format!("Baking archetype images:\n    {}", archetype_desc).as_str());
    let result = Input::<String>::new()
        .with_prompt("Type yes to bake images")
        .allow_empty(true)
        .interact_text();
    match result {
        Ok(approval) => approval == "yes",
        Err(e) => task::Error::from(e).exit(),
    }
}
