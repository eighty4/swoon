use std::path::PathBuf;

use crate::api::{OperatingSystem, task};
use crate::api::process::Process;
use crate::api::util::split_string;

// todo check if cli is up to date
//  gcloud version

// todo check if user is authed

const ERR_DEFAULT_PROJECT: &str = r"gcloud does not have a configured default project

run 'gcloud configure set core/project $my_project_id'";

pub struct GcloudCli {
    path: PathBuf,
}

impl GcloudCli {
    pub fn new(path: PathBuf) -> Self {
        GcloudCli {
            path,
        }
    }

    pub async fn all_projects(&self) -> task::Result<Vec<String>> {
        let output = Process::invoke(&self.path, [
            "projects",
            "list",
            "--format=value(projectId)",
            "-q",
        ])?;
        Ok(split_string("\n", output))
    }

    pub async fn default_project_id(&self) -> task::Result<String> {
        let default_project_id = Process::invoke(&self.path, [
            "config",
            "get-value",
            "core/project",
            "-q",
        ])?;
        if default_project_id.eq("(unset)") {
            task::Error::result(ERR_DEFAULT_PROJECT)
        } else {
            Ok(default_project_id)
        }
    }

    pub async fn image_name_by_os(&self, os: &OperatingSystem) -> task::Result<String> {
        Process::invoke(&self.path, [
            "compute",
            "images",
            "list",
            "--format=value(name)",
            format!("--filter=family:{}", image_family_name_by_os(os)).as_ref(),
            "-q",
        ])
    }
}

// pub async fn all_image_names(ctx: &SwoonContext) -> task::Result<Vec<String>> {
//     let org_name = match &ctx.config {
//         Some(cfg) => &cfg.org_name,
//         None => return task::Error::result("no config"),
//     };
//     let output = Process::invoke(&ctx.gcloud_path(), [
//         "compute",
//         "images",
//         "list",
//         "--no-standard-images",
//         "--format=value(NAME)",
//         format!("--filter=name:{}-{}-", org_name, "archetype").as_ref(),
//         "-q",
//     ])?;
//     Ok(split_string("\n", output))
// }

fn image_family_name_by_os(os: &OperatingSystem) -> String {
    match os {
        OperatingSystem::Debian { version } => format!("debian-{}", version),
        OperatingSystem::Ubuntu { version, minimal } => if *minimal {
            format!("ubuntu-minimal-{}{}-lts", version.major, version.minor)
        } else {
            format!("ubuntu-{}{}-lts", version.major, version.minor)
        },
    }
}

// #[cfg(test)]
// mod tests {
//     use which::which;
//
//     use crate::api::{CloudPlatform, DEBIAN_11, DEFAULT_OS};
//     use crate::api::config::SwoonConfig;
//     use crate::api::context::SwoonContext;
//
//     use super::*;
//
//     #[test]
//     async fn test_default_project_id() {
//         let gcloud_cli = GcloudCli::new(which("gcloud").unwrap());
//         let project_id = gcloud_cli.default_project_id().await.unwrap();
//         assert_eq!("example-project-id", project_id);
//     }
//
//     #[test]
//     fn test_image_name_by_os() {
//         let gcloud_cli = GcloudCli::new(which("gcloud").unwrap());
//         let image_name = gcloud_cli.image_name_by_os(&DEBIAN_11).unwrap();
//         assert_eq!("debian-11-bullseye-v20211209", image_name);
//     }
// }
