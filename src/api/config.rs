use std::fs;
use std::io::Write;

use yaml_rust::YamlLoader;

use crate::api::{CloudPlatform, DEFAULT_OS, OperatingSystem, task};
use crate::api::template::{Template, template_object};

pub struct SwoonConfig {
    pub org_name: String,
    pub default_os: OperatingSystem,
    pub default_platform: CloudPlatform,
}

pub fn write_config(tmpl_name: Option<&str>, cfg: &SwoonConfig) -> task::Result<()> {
    let tmpl = Template::new(include_bytes!("swoon.yml.liquid"))?;
    let cfg_content: String = tmpl.render(&template_object!({
        "org_name": cfg.org_name,
        "default_os": cfg.default_os.to_string(),
        "default_platform": cfg.default_platform.to_str(),
    }))?;
    println!("Writing a {} swoon config for {}", tmpl_name.unwrap_or("default"), cfg.org_name);
    let mut file = fs::File::create("swoon.yml")?;
    file.write_all(cfg_content.as_bytes())?;
    task::SUCCESS
}

pub fn read_config(config_path: &str) -> task::Result<SwoonConfig> {
    let config_read = fs::read_to_string(config_path)?;
    parse_config(config_read.as_ref())
}

fn parse_config(config: &str) -> task::Result<SwoonConfig> {
    let yaml_read = YamlLoader::load_from_str(config);
    if let Err(e) = yaml_read {
        return task::Error::result(e.to_string().as_ref());
    }
    let yaml_docs = yaml_read.unwrap();
    let doc = &yaml_docs[0];
    let org_name = doc["org_name"].as_str()
        .expect("org_name is a required swoon.yml field").to_string();
    let default_platform = CloudPlatform::GCP;
    let default_os = match doc["default_os"].as_str() {
        None => DEFAULT_OS,
        Some(s) => OperatingSystem::from_string(s)?,
    };
    Ok(SwoonConfig {
        org_name,
        default_os,
        default_platform,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minimal_config() {
        let config = parse_config("---\norg_name: eighty4").unwrap();
        assert_eq!(config.org_name, "eighty4");
        assert_eq!(config.default_platform, CloudPlatform::GCP);
        assert_eq!(config.default_os, DEFAULT_OS);
    }

    #[test]
    fn test_parse_full_config() {
        let config_str = r"---
        org_name: eighty4
        default_os: debian:9
        default_platform: aws
        ";
        let config = parse_config(config_str).unwrap();
        assert_eq!(config.org_name, "eighty4");
        assert_eq!(config.default_platform, CloudPlatform::GCP);
        assert_eq!(config.default_os, OperatingSystem::Debian { version: 9 });
    }
}
