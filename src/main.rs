use std::process::exit;

use clap::{App, Arg, ArgMatches, SubCommand};

use crate::api::{swoon_error_result, SwoonError};
use crate::api::context::{init_swoon_context, SwoonContext};
use crate::bake::{bake_machine_images, BakeOpts};
use crate::init::{init_swoon_project, InitOpts};

mod ansible;
mod api;
mod bake;
mod gcloud;
mod init;
mod packer;

fn main() {
    let c: SwoonContext = init_swoon_context().expect("failed to initialize");

    let a: ArgMatches = App::new("Swoon CLI")
        .version("0.0.1")
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
        .get_matches();

    let r: Result<(), SwoonError> = exec_cmd(&c, &a);
    if let Some(err) = r.err() {
        c.terminal.write_line(&*err.to_string()).unwrap();
        if a.subcommand_name() != None { exit(1) }
    }
}

fn exec_cmd(c: &SwoonContext, a: &ArgMatches) -> Result<(), SwoonError> {
    match a.subcommand() {
        ("init", Some(init_match)) => init_swoon_project(&c, &InitOpts {
            template_name: init_match.value_of("template"),
        }),
        ("bake", Some(bake_match)) => bake_machine_images(&c, &BakeOpts {
            approve_plan: bake_match.is_present("approve-plan"),
        }),
        _ => swoon_error_result("No subcommand given.\nTry `swoon init`."),
    }
}
