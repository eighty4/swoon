use std::process::exit;

use clap;

use crate::api::command;
use crate::api::context::SwoonContext;
use crate::bake::{bake_machine_images, BakeOpts};
use crate::init::{init_swoon_project, InitOpts};

mod ansible;
mod api;
mod bake;
mod images;
mod init;
mod packer;
mod platforms;

fn main() {
    let a: clap::ArgMatches = clap::Command::new("Swoon CLI")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Adam McKee <adam.be.g84d@gmail.com>")
        .about("The cloud for you, not for the enterprise.")

        .arg(clap::Arg::new("debug")
            .short('d')
            .long("debug")
            .help("Print extra debugging info")
            .takes_value(false))

        .subcommand(clap::Command::new("init")
            .about("init your cloud config")

            .arg(clap::Arg::new("non-interactive")
                .long("non-interactive")
                .help("Init project without user input")
                .takes_value(false))

            .arg(clap::Arg::new("org-name")
                .long("org-name")
                .value_name("ORGANIZATION")
                .help("Specify swoon.yml template")
                .takes_value(true))

            .arg(clap::Arg::new("template")
                .short('t')
                .long("template")
                .value_name("TEMPLATE")
                .help("Specify swoon.yml template")
                .takes_value(true))

            .arg(clap::Arg::new("cloud-platform")
                .long("cloud-platform")
                .value_name("PLATFORM")
                .help("Cloud platform")
                .possible_values(&["gcp"])
                .takes_value(true))

            .arg(clap::Arg::new("operating-system")
                .long("operating-system")
                .value_name("OS")
                .help("Default operating system for pre-baked machine images and servers")
                .takes_value(true))
        )
        .subcommand(clap::Command::new("bake")
            .about("bake your machine images")

            .arg(clap::Arg::new("approve-plan")
                .short('a')
                .long("approve-plan")
                .help("Approve machine image plan")
                .takes_value(false))
        )
        .get_matches();

    let c: SwoonContext = SwoonContext::init_from_args(&a)
        .expect("failed to initialize");

    let r: command::Result = exec_cmd(&c, &a);
    if let Some(err) = r.err() {
        let error_msg = &err.cause.to_string();
        if err.alt_commands.len() > 0 {
            c.write_line(format!("{}. Try these commands:", error_msg).as_str());
            for alt_command in err.alt_commands {
                c.write_line(format!("    swoon {0}\n    swoon help {0}", alt_command).as_str());
            }
        } else {
            c.write_line(error_msg);
        }
        exit(1);
    }
}

fn exec_cmd(ctx: &SwoonContext, args: &clap::ArgMatches) -> command::Result {
    match args.subcommand() {
        None => command::SUCCESS,
        Some((subcommand_name, subcommand_args)) => {
            match subcommand_name {
                "init" => init_swoon_project(&ctx, &InitOpts {
                    non_interactive: subcommand_args.is_present("non-interactive"),
                    template_name: subcommand_args.value_of("template"),
                    org_name: subcommand_args.value_of("org-name"),
                    default_platform: subcommand_args.value_of("cloud-platform"),
                    default_os: subcommand_args.value_of("operating-system"),
                }),
                "bake" => bake_machine_images(&ctx, &BakeOpts {
                    approve_plan: subcommand_args.is_present("approve-plan"),
                }),
                &_ => command::SUCCESS,
            }
        }
    }
}
