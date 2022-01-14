use std::process;

use futures::executor;

use gcloud::GcloudContext;

use crate::api::binaries::{BinaryPaths, PathLookup};
use crate::api::config::SwoonConfig;

pub mod gcloud;

#[derive(Clone)]
pub struct PlatformContexts {
    pub gcloud: Option<GcloudContext>,
}

impl PlatformContexts {
    pub fn default() -> Self {
        Self {
            gcloud: None,
        }
    }

    pub fn init(binary_paths: &BinaryPaths, config_opt: &Option<SwoonConfig>) -> Self {
        let config = match config_opt {
            None => return Self::default(),
            Some(cfg) => cfg,
        };
        let gcloud_ctx_result = executor::block_on(GcloudContext::init(
            binary_paths.gcloud_path(), &config));
        let gcloud = match gcloud_ctx_result {
            Ok(gcloud) => Some(gcloud),
            Err(e) => {
                // todo why can't i use error_exit here (futures lose Error trait compat?)
                println!("{}", e.to_string());
                dbg!(e);
                process::exit(1);
            }
        };
        Self {
            gcloud,
        }
    }
}
