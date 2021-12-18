use std::fs::File;
use std::io::Write;

use liquid::Template;

use crate::api::SwoonError;

const DEFAULT_TEMPLATE_NAME: &str = "minimal";

pub struct ConfigModel {
    pub org_name: String,
}

fn cfg_tmpl_by_name(tmpl_name: &str) -> String {
    let cfg_tmpl_bytes;
    match tmpl_name {
        "full" => cfg_tmpl_bytes = include_bytes!("swoon.full.yml.liquid"),
        "minimal" | &_ => cfg_tmpl_bytes = include_bytes!("swoon.minimal.yml.liquid"),
    }
    return String::from_utf8_lossy(cfg_tmpl_bytes).to_string();
}

pub fn write_config(tmpl_name: Option<&str>, cfg_model: &ConfigModel) -> Result<(), SwoonError> {
    let tmpl_name = tmpl_name.unwrap_or(DEFAULT_TEMPLATE_NAME);
    let cfg_tmpl_parser = liquid::ParserBuilder::with_stdlib().build()?;
    let cfg_tmpl: Template = cfg_tmpl_parser.parse(cfg_tmpl_by_name(tmpl_name).as_ref())?;
    let cfg_content: String = cfg_tmpl.render(&liquid::object!({
        "org_name": cfg_model.org_name,
    }))?;
    println!("Creating a {} swoon config for {}", tmpl_name, cfg_model.org_name);
    let mut file = File::create("swoon.yml")?;
    file.write_all(cfg_content.as_bytes())?;
    return Ok(());
}
