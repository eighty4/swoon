use std::fs;
use std::io::Write;
use std::process::exit;

use yaml_rust::YamlLoader;

use crate::api::SwoonError;
use crate::api::template::{Template, template_object};
use crate::gcloud::cli::{DEFAULT_OS, GcpImageFamily};

const DEFAULT_TEMPLATE_NAME: &str = "minimal";

pub struct SwoonConfig {
    pub org_name: String,
    pub default_os: GcpImageFamily,
}

fn cfg_tmpl_by_name(tmpl_name: &str) -> String {
    let cfg_tmpl_bytes;
    match tmpl_name {
        "full" => cfg_tmpl_bytes = include_bytes!("swoon.full.yml.liquid"),
        "minimal" | &_ => cfg_tmpl_bytes = include_bytes!("swoon.minimal.yml.liquid"),
    }
    return String::from_utf8_lossy(cfg_tmpl_bytes).to_string();
}

pub fn write_config(tmpl_name: Option<&str>, cfg: &SwoonConfig) -> Result<(), SwoonError> {
    let tmpl_name = tmpl_name.unwrap_or(DEFAULT_TEMPLATE_NAME);
    let tmpl = Template::new(cfg_tmpl_by_name(tmpl_name).as_ref())?;
    let cfg_content: String = tmpl.render(&template_object!({
        "org_name": cfg.org_name,
    }))?;
    println!("Creating a {} swoon config for {}", tmpl_name, cfg.org_name);
    let mut file = fs::File::create("swoon.yml")?;
    file.write_all(cfg_content.as_bytes())?;
    return Result::Ok(());
}

pub fn read_config(config_path: &str) -> Option<SwoonConfig> {
    let config_read = fs::read_to_string(config_path);
    if config_read.is_err() {
        return None;
    }
    return parse_config(config_read.unwrap().as_ref());
}

fn parse_config(config: &str) -> Option<SwoonConfig> {
    let yaml_read = YamlLoader::load_from_str(config);
    if yaml_read.is_err() {
        return None;
    }
    let docs = yaml_read.unwrap();
    let doc = &docs[0];
    let org_name = doc["org_name"].as_str().unwrap().to_string();
    let default_os_str = doc["default_os"].as_str();
    let parsed_default_os = parse_default_os(default_os_str);
    if parsed_default_os.is_err() {
        println!("unable to parse {} as an os specifier", default_os_str.unwrap());
        exit(1);
    }
    let default_os = parsed_default_os.unwrap();
    return Some(SwoonConfig {
        org_name,
        default_os,
    });
}

fn parse_default_os(so: Option<&str>) -> Result<GcpImageFamily, SwoonError> {
    match so {
        None => Result::Ok(DEFAULT_OS),
        Some(s) => GcpImageFamily::from_string(s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let config = parse_config("---\norg_name: eighty4").unwrap();
        assert_eq!(config.org_name, "eighty4");
    }
}
