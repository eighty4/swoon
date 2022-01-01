use std::collections::HashMap;
use std::path::PathBuf;

use console::Term;
use which::which;

use crate::api::config::{read_config, SwoonConfig};
use crate::api::task;

#[derive(PartialEq, Eq, Hash)]
pub enum Binary {
    GCLOUD
}

impl Clone for Binary {
    fn clone(&self) -> Self {
        match self {
            Binary::GCLOUD => Binary::GCLOUD,
        }
    }
}

pub trait BinaryPaths {
    fn gcloud_bin_path(&self) -> &PathBuf;
    fn bin_path(&self, bin: Binary) -> &PathBuf;
}

pub struct SwoonContext {
    pub binaries: HashMap<Binary, PathBuf>,
    pub config: Option<SwoonConfig>,
    pub config_path: Option<PathBuf>,
    pub terminal: Term,
}

impl SwoonContext {
    pub fn init() -> task::Result<SwoonContext> {
        let gcloud_path = which("gcloud");
        let mut binaries = HashMap::new();
        if gcloud_path.is_ok() {
            binaries.insert(Binary::GCLOUD, gcloud_path.unwrap());
        }
        let config_path_buf = std::env::current_dir()?.join("swoon.yml");
        let mut config_path = None;
        let mut config = None;
        if config_path_buf.exists() {
            config = Some(read_config(config_path_buf.to_str().unwrap())?);
            config_path = Some(config_path_buf);
        }
        Ok(SwoonContext {
            binaries,
            config,
            config_path,
            terminal: Term::stdout(),
        })
    }

    pub fn with_config(&self, config: SwoonConfig) -> Self {
        Self {
            binaries: self.binaries.clone(),
            config: Some(config),
            config_path: self.config_path.clone(),
            terminal: Term::stdout(),
        }
    }
}

impl BinaryPaths for SwoonContext {
    fn gcloud_bin_path(&self) -> &PathBuf {
        self.binaries.get(&Binary::GCLOUD).expect("wtf")
    }

    fn bin_path(&self, bin: Binary) -> &PathBuf {
        match bin {
            Binary::GCLOUD => BinaryPaths::gcloud_bin_path(self),
        }
    }
}
