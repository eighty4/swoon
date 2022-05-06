use dialoguer::{Input, Select};

use crate::ansible;
use crate::api::{CloudPlatform, command, DEFAULT_OS, OperatingSystem, task};
use crate::api::CloudPlatform::*;
use crate::api::command::Name::Bake;
use crate::api::config::SwoonConfig;
use crate::api::context::SwoonContext;
use crate::api::util::DataDir;
use crate::packer::PackerBuild;

pub struct InitOpts<'a> {
    pub non_interactive: bool,
    pub template_name: Option<&'a str>,
    pub org_name: Option<&'a str>,
    pub default_platform: Option<&'a str>,
    pub default_os: Option<&'a str>,
}

pub fn init_swoon_project(ctx: &SwoonContext, opts: &InitOpts) -> command::Result {
    if ctx.has_config() {
        // todo[speed] build platform ctx for this error is unnecessary
        return command::Error::with_command_suggestions(
            "A swoon.yml file already exists in your current directory",
            vec!(Bake),
        );
    }
    DataDir::init()?;

    let new_ctx = &ctx.with_config(build_project_config(opts)?);
    new_ctx.config().write(opts.template_name)?;

    ansible::init_roles_dir()?;

    PackerBuild::default_archetype(new_ctx).write_config_files()?;

    command::SUCCESS
}

fn build_project_config(opts: &InitOpts) -> task::Result<SwoonConfig> {
    if opts.non_interactive {
        resolve_config_from_opts(opts)
    } else {
        prompt_for_config(opts)
    }
}

fn resolve_config_from_opts(opts: &InitOpts) -> task::Result<SwoonConfig> {
    let org_name = match opts.org_name {
        None => return task::Error::result("--org-name is required"),
        Some(s) => String::from(s),
    };
    let default_os = match opts.default_os {
        None => DEFAULT_OS,
        Some(s) => OperatingSystem::from_string(s)?,
    };
    let default_platform = match opts.default_platform {
        None => GCP, // todo[multi-platform] resolve based on which platform cli installations are found
        Some(s) => CloudPlatform::from_str(s),
    };
    Ok(SwoonConfig {
        org_name,
        default_os,
        default_platform,
    })
}

fn prompt_for_config(opts: &InitOpts) -> task::Result<SwoonConfig> {
    let org_name = match opts.org_name {
        None => prompt_for_org_name(),
        Some(s) => String::from(s),
    };
    let default_os = match opts.default_os {
        None => prompt_for_default_os(&org_name)?,
        Some(s) => OperatingSystem::from_string(s)?,
    };
    let default_platform = match opts.default_platform {
        None => prompt_for_default_platform(&org_name)?,
        Some(s) => CloudPlatform::from_str(s),
    };
    Ok(SwoonConfig {
        org_name,
        default_os,
        default_platform,
    })
}

fn prompt_for_org_name() -> String {
    let org_name = Input::<String>::new()
        .with_prompt("What is your organization name?")
        .interact_text();
    match org_name {
        Ok(s) => s,
        Err(e) => task::Error::from(e).exit(),
    }
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
        Some(i) => Ok(CloudPlatform::from_str(platform_opts[i])),
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
