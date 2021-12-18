use std::collections::HashMap;
use std::path::PathBuf;

use console::Term;
use which::which;

use crate::api::SwoonError;

#[derive(PartialEq, Eq, Hash)]
pub enum Binary {
    GCLOUD
}

pub trait BinaryPaths {
    fn gcloud_bin_path(&self) -> &PathBuf;
    fn bin_path(&self, bin: Binary) -> &PathBuf;
}

pub struct SwoonContext {
    pub binaries: HashMap<Binary, PathBuf>,
    pub config_path: Option<PathBuf>,
    pub terminal: Term,
}

impl BinaryPaths for SwoonContext {
    fn gcloud_bin_path(&self) -> &PathBuf {
        return self.binaries.get(&Binary::GCLOUD).expect("wtf");
    }

    fn bin_path(&self, bin: Binary) -> &PathBuf {
        match bin {
            Binary::GCLOUD => BinaryPaths::gcloud_bin_path(self),
        }
    }
}

pub fn init_swoon_context() -> std::result::Result<SwoonContext, SwoonError> {
    let gcloud_path = which("gcloud");
    let mut binaries = HashMap::new();
    if gcloud_path.is_ok() {
        binaries.insert(Binary::GCLOUD, gcloud_path.unwrap());
    }

    // todo handle error
    let config_path_buf = std::env::current_dir().unwrap().join("swoon.yml");
    let mut config_path = None;
    if config_path_buf.exists() {
        config_path = Some(config_path_buf);
    }
    return std::result::Result::Ok(SwoonContext {
        binaries,
        config_path,
        terminal: Term::stdout(),
    });
}
