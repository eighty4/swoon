use dialoguer::{Input, Select};

use crate::{ansible, packer};
use crate::api::{CloudPlatform, command, DEFAULT_OS, OperatingSystem, task};
use crate::api::CloudPlatform::*;
use crate::api::command::Name::Bake;
use crate::api::config::{SwoonConfig, write_config};
use crate::api::context::SwoonContext;

pub struct InitOpts<'a> {
    pub template_name: Option<&'a str>,
}

pub fn init_swoon_project(ctx: &SwoonContext, opts: &InitOpts) -> command::Result {
    if let Some(_) = ctx.config_path {
        return command::Error::with_command_suggestions(
            "A swoon.yml file already exists in your current directory",
            vec!(Bake),
        );
    }
    let config = prompt_for_config()?;
    write_config(opts.template_name, &config)?;
    let ctx_with_cfg = ctx.with_config(config);
    write_gcp_archetype_baking_config(&ctx_with_cfg)?;
    command::SUCCESS
}

fn prompt_for_config() -> task::Result<SwoonConfig> {
    let org_name: String = prompt_for_org_name()?;
    let default_os = prompt_for_default_os(&org_name)?;
    let default_platform: CloudPlatform = prompt_for_default_platform(&org_name)?;
    Ok(SwoonConfig {
        org_name,
        default_os,
        default_platform,
    })
}

fn prompt_for_org_name() -> task::Result<String> {
    let org_name = Input::<String>::new()
        .with_prompt("What is your organization name?")
        .interact_text()?;
    Ok(org_name)
}

fn prompt_for_default_platform(org_name: &String) -> task::Result<CloudPlatform> {
    let platform_opts = vec![
        GCP.to_str(),
        AWS.to_str(),
    ];
    let platform_selection = Select::new()
        .with_prompt(format!("What is {}'s primary cloud platform?", org_name))
        .items(&platform_opts)
        .default(0)
        .interact_opt()?;
    match platform_selection {
        Some(i) => Ok(CloudPlatform::from_str(platform_opts[i]).unwrap()),
        None => task::Error::result("no cloud platform selected"),
    }
}

fn prompt_for_default_os(org_name: &String) -> task::Result<OperatingSystem> {
    let os_opts = vec![
        "debian:11",
        "ubuntu:20.04:minimal",
        "ubuntu:20.04",
    ];
    let os_selection = Select::new()
        .with_prompt(format!("What is {}'s default cloud operating system?", org_name))
        .items(&os_opts)
        .default(0)
        .interact_opt()?;
    match os_selection {
        Some(i) => OperatingSystem::from_string(os_opts[i]),
        None => Ok(DEFAULT_OS),
    }
}

fn write_gcp_archetype_baking_config(ctx: &SwoonContext) -> task::Result<()> {
    packer::write_gcp_archetype_config(ctx)?;
    ansible::write_archetype_playbook()?;
    task::SUCCESS
}
