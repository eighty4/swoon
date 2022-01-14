use std::path::PathBuf;

use crate::api::config::SwoonConfig;
use crate::api::task;
use crate::platforms::gcloud::cli::GcloudCli;

pub mod cli;

#[derive(Clone)]
pub struct GcloudContext {
    pub default_project_id: String,
}

impl GcloudContext {
    pub async fn init(path: PathBuf, cfg: &SwoonConfig) -> task::Result<Self> {
        let gcloud_cli = GcloudCli::new(path);

        let default_project_id = gcloud_cli.default_project_id().await?;

        Ok(Self {
            default_project_id,
        })
    }
}
