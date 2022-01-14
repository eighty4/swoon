use std::process::exit;

use clap::{App, Arg, ArgMatches, SubCommand};

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
    let a: ArgMatches = App::new("Swoon CLI")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Adam McKee <adam.be.g84d@gmail.com>")
        .about("The cloud for you, not for the enterprise.")
        .subcommand(SubCommand::with_name("init")
            .about("init your cloud config")
            .arg(Arg::with_name("template")
                .short("t")
                .long("template")
                .value_name("TEMPLATE")
                .help("Specify swoon.yml template")
                .takes_value(true))
        )
        .subcommand(SubCommand::with_name("bake")
            .about("bake your machine images")
            .arg(Arg::with_name("approve-plan")
                .short("a")
                .long("approve-plan")
                .help("Approve machine image plan")
                .takes_value(false))
        )
        .arg(Arg::with_name("debug")
            .short("d")
            .long("debug")
            .help("Print extra debugging info")
            .takes_value(false))
        .get_matches();

    let c: SwoonContext = SwoonContext::init_from_args(&a).expect("failed to initialize");

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

fn exec_cmd(ctx: &SwoonContext, args: &ArgMatches) -> command::Result {
    match args.subcommand() {
        ("init", Some(init_match)) => init_swoon_project(&ctx, &InitOpts {
            template_name: init_match.value_of("template"),
        }),
        ("bake", Some(bake_match)) => bake_machine_images(&ctx, &BakeOpts {
            approve_plan: bake_match.is_present("approve-plan"),
        }),
        _ => command::SUCCESS,
    }
}
