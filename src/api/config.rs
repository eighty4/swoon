use std::fs;
use std::path::{Path, PathBuf};

use yaml_rust::YamlLoader;

use crate::api::{CloudPlatform, DEFAULT_OS, OperatingSystem, task};
use crate::api::output::file::{Directory, File};
use crate::api::output::template::{Template, template_object, TemplateFile};
use crate::api::util::ProjectDir;

#[derive(Clone)]
pub struct SwoonConfig {
    pub org_name: String,
    pub default_os: OperatingSystem,
    pub default_platform: CloudPlatform,
}

impl SwoonConfig {
    pub fn config_file_path() -> PathBuf {
        ProjectDir::path().join("swoon.yml")
    }

    pub fn read_from_current_dir() -> task::Result<Option<SwoonConfig>> {
        let config_path = Self::config_file_path();
        let config = if config_path.exists() && config_path.is_file() {
            Some(Self::read(config_path)?)
        } else {
            None
        };
        Ok(config)
    }

    fn parse(config: &str) -> task::Result<SwoonConfig> {
        let yaml_read = YamlLoader::load_from_str(config);
        if let Err(e) = yaml_read {
            return task::Error::result(e.to_string());
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

    pub fn read<P: AsRef<Path>>(config_path: P) -> task::Result<SwoonConfig> {
        let config_read = fs::read_to_string(config_path)?;
        Self::parse(config_read.as_ref())
    }

    pub fn write(&self, tmpl_name_opt: Option<&str>) -> task::Result<()> {
        let tmpl_name = tmpl_name_opt.unwrap_or("default").to_string();
        println!("Writing a {} swoon config for {}", tmpl_name, self.org_name);
        SwoonConfigFile {
            cfg: self.clone(),
            tmpl_name,
        }.write()
    }
}

pub struct SwoonConfigFile {
    cfg: SwoonConfig,
    tmpl_name: String,
}

impl TemplateFile for SwoonConfigFile {
    fn data(&self) -> task::Result<liquid::Object> {
        Ok(template_object!({
            "org_name": self.cfg.org_name,
            "default_os": self.cfg.default_os.to_string(),
            "default_platform": self.cfg.default_platform.to_str(),
        }))
    }

    fn template(&self) -> task::Result<Template> {
        Template::new(include_bytes!("swoon.yml.liquid"))
    }

    fn template_output_path(&self) -> (Directory, String) {
        (Directory::ProjectRoot, "swoon.yml".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minimal_config() {
        let config = SwoonConfig::parse("---\norg_name: eighty4").unwrap();
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
        let config = SwoonConfig::parse(config_str).unwrap();
        assert_eq!(config.org_name, "eighty4");
        assert_eq!(config.default_platform, CloudPlatform::GCP);
        assert_eq!(config.default_os, OperatingSystem::Debian { version: 9 });
    }
}
