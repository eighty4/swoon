use std::collections::HashMap;
use std::path::PathBuf;

use clap::ArgMatches;
use console::Term;
use which::which;

use crate::api::config::SwoonConfig;
use crate::api::task;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Binary {
    GCLOUD,
    PACKER,
}

pub trait BinaryPaths {
    fn bin_path(&self, bin: Binary) -> &PathBuf;
    fn gcloud_bin_path(&self) -> &PathBuf;
    fn packer_bin_path(&self) -> &PathBuf;
}

#[derive(Clone)]
pub struct SwoonOpts {
    pub(crate) debug: bool,
}

pub struct SwoonContext {
    pub binaries: HashMap<Binary, PathBuf>,
    pub config: Option<SwoonConfig>,
    pub config_path: Option<PathBuf>,
    pub opts: SwoonOpts,
    pub terminal: Term,
}

impl SwoonContext {
    #[cfg(test)]
    pub fn default() -> task::Result<Self> {
        Self::init(SwoonOpts {
            debug: false,
        })
    }

    pub fn init_from_args(args: &ArgMatches) -> task::Result<Self> {
        Self::init(SwoonOpts {
            debug: args.is_present("debug"),
        })
    }

    pub fn init(opts: SwoonOpts) -> task::Result<Self> {
        let gcloud_path = which("gcloud");
        let mut binaries = HashMap::new();
        if gcloud_path.is_ok() {
            binaries.insert(Binary::GCLOUD, gcloud_path.unwrap());
        }
        let config_path_buf = std::env::current_dir()?.join("swoon.yml");
        let mut config_path = None;
        let mut config = None;
        if config_path_buf.exists() {
            config = Some(SwoonConfig::read(config_path_buf.to_str().unwrap())?);
            config_path = Some(config_path_buf);
        }
        Ok(SwoonContext {
            binaries,
            config,
            config_path,
            opts,
            terminal: Term::stdout(),
        })
    }

    pub fn with_config(&self, config: SwoonConfig) -> Self {
        Self {
            binaries: self.binaries.clone(),
            config: Some(config),
            config_path: self.config_path.clone(),
            opts: self.opts.clone(),
            terminal: Term::stdout(),
        }
    }
}

impl BinaryPaths for SwoonContext {
    fn bin_path(&self, bin: Binary) -> &PathBuf {
        match bin {
            Binary::GCLOUD => BinaryPaths::gcloud_bin_path(self),
            Binary::PACKER => BinaryPaths::packer_bin_path(self),
        }
    }

    fn gcloud_bin_path(&self) -> &PathBuf {
        self.binaries.get(&Binary::GCLOUD).unwrap()
    }

    fn packer_bin_path(&self) -> &PathBuf {
        self.binaries.get(&Binary::PACKER).unwrap()
    }
}
