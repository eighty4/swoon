use std::collections::HashMap;
use std::path::PathBuf;

use futures::join;

use crate::api::{OperatingSystem, task};
use crate::api::config::SwoonConfig;
use crate::platforms::gcloud::cli::GcloudCli;

pub mod cli;

#[derive(Clone)]
pub struct GcloudContext {
    pub default_os_image_name: String,
    pub default_project_id: String,
    pub image_names_by_os: HashMap<OperatingSystem, String>,
}

impl GcloudContext {
    pub async fn init(path: PathBuf, cfg: &SwoonConfig) -> task::Result<Self> {
        let gcloud_cli = GcloudCli::new(path);
        let (
            all_projects_result,
            default_os_image_name_result,
            default_project_id_result,
        ) = join!(
            gcloud_cli.all_projects(),
            gcloud_cli.image_name_by_os(&cfg.default_os),
            gcloud_cli.default_project_id(),
        );

        let all_projects: Vec<String> = all_projects_result?;
        let default_os_image_name: String = default_os_image_name_result?;
        let default_project_id = default_project_id_result?;

        if !all_projects.contains(&default_project_id) {
            return task::Error::result(format!(
                "{} is not a project for your authed gcloud session", default_project_id));
        }

        let mut image_names_by_os: HashMap<OperatingSystem, String> = HashMap::new();
        image_names_by_os.insert((&cfg.default_os).clone(), default_os_image_name.clone());

        Ok(Self {
            default_os_image_name,
            default_project_id,
            image_names_by_os,
        })
    }
}
