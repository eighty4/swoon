use std::path::PathBuf;

use futures::join;

use crate::api::task;
use crate::api::config::SwoonConfig;
use crate::platforms::gcloud::cli::GcloudCli;

pub mod cli;
pub mod images;
pub mod packer;

#[derive(Clone)]
pub struct GcloudContext {
    pub default_project_id: String,
}

impl GcloudContext {
    pub async fn init(path: PathBuf, cfg: &SwoonConfig) -> task::Result<Self> {
        let gcloud_cli = GcloudCli::new(path);
        let (
            all_projects_result,
            default_project_id_result,
        ) = join!(
            gcloud_cli.all_projects(),
            gcloud_cli.default_project_id(),
        );

        let all_projects: Vec<String> = all_projects_result?;
        let default_project_id = default_project_id_result?;

        if !all_projects.contains(&default_project_id) {
            return task::Error::result(format!(
                "{} is not a project for your authed gcloud session", default_project_id));
        }

        Ok(Self {
            default_project_id,
        })
    }
}
