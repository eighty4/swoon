use dialoguer::Input;

use crate::api::{swoon_error_result, SwoonError};
use crate::api::config::{ConfigModel, write_config};
use crate::api::context::SwoonContext;

pub struct InitOpts<'a> {
    pub template_name: Option<&'a str>,
}

pub fn init_swoon_project(ctx: &SwoonContext, opts: &InitOpts) -> Result<(), SwoonError> {
    if let Some(_) = ctx.config_path {
        return swoon_error_result("A swoon.yml already exists in the current dir.");
    }

    let org_name: String = Input::new()
        .with_prompt("What is your organization name?")
        .interact_text()?;

    write_config(opts.template_name, &ConfigModel {
        org_name,
    })?;
    return Result::Ok(());
}
