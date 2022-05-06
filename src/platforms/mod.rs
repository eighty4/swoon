use futures::executor;

use gcloud::GcloudContext;

use crate::api::binaries::{BinaryPaths, PathLookup};
use crate::api::config::SwoonConfig;

pub mod gcloud;
pub mod packer;

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
            Err(e) => e.exit(),
        };
        Self {
            gcloud,
        }
    }

    pub fn gcloud_ctx(&self) -> &GcloudContext {
        self.gcloud.as_ref().expect("no gcloud ctx")
    }
}
