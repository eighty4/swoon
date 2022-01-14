use std::path::PathBuf;

use crate::api::{OperatingSystem, task};
use crate::api::binaries::PathLookup;
use crate::api::context::SwoonContext;
use crate::api::process::Process;
use crate::api::util::split_string;

// todo check if cli is up to date
//  gcloud version

// todo check if user is authed

// todo check if gcloud permissions and apis enabled
//  gcloud.compute.images.list

pub struct GcloudCli {
    path: PathBuf,
}

impl GcloudCli {
    pub fn new(path: PathBuf) -> Self {
        GcloudCli {
            path,
        }
    }

    pub async fn default_project_id(&self) -> task::Result<String> {
        Process::invoke(&self.path, ["config", "get-value", "core/project", "-q"])
    }
}

pub async fn all_image_names(ctx: &SwoonContext) -> task::Result<Vec<String>> {
    let org_name = match &ctx.config {
        Some(cfg) => &cfg.org_name,
        None => return task::Error::result("no config"),
    };
    let output = Process::invoke(&ctx.gcloud_path(), [
        "compute",
        "images",
        "list",
        "--no-standard-images",
        "--format=value(NAME)",
        format!("--filter=name:{}-{}-", org_name, "archetype").as_ref(),
        "-q",
    ])?;
    Ok(split_string("\n", output))
}

pub fn default_project_id(ctx: &SwoonContext) -> task::Result<String> {
    Process::invoke(&ctx.gcloud_path(), ["config", "get-value", "core/project", "-q"])
}

pub fn image_name_by_os(ctx: &SwoonContext, os: OperatingSystem) -> task::Result<String> {
    Process::invoke(&ctx.gcloud_path(), [
        "compute",
        "images",
        "list",
        "--format=value(NAME)",
        format!("--filter=family:{}", image_family_name_by_os(os)).as_ref(),
        "-q",
    ])
}

fn image_family_name_by_os(os: OperatingSystem) -> String {
    match os {
        OperatingSystem::Debian { version } => format!("debian-{}", version),
        OperatingSystem::Ubuntu { version, minimal } => if minimal {
            format!("ubuntu-minimal-{}-lts", version * 100f32)
        } else {
            format!("ubuntu-{}-lts", version * 100f32)
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::api::{CloudPlatform, DEFAULT_OS};
    use crate::api::config::SwoonConfig;

    use super::*;

    #[test]
    fn test_all_image_names() {
        let ctx = SwoonContext::default().unwrap().with_config(SwoonConfig {
            org_name: "eighty4".to_string(),
            default_os: DEFAULT_OS,
            default_platform: CloudPlatform::GCP,
        });
        let image_names = all_image_names(&ctx).unwrap();
        assert_eq!(1, image_names.len());
    }

    #[test]
    fn test_default_project_id() {
        let ctx = SwoonContext::default().unwrap();
        let project_id = default_project_id(&ctx).unwrap();
        assert_eq!("example-project-id", project_id);
    }

    #[test]
    fn test_image_name_by_os() {
        let ctx = SwoonContext::default().unwrap();
        let os = OperatingSystem::Debian { version: 11 };
        let image_name = image_name_by_os(&ctx, os).unwrap();
        assert_eq!("debian-11-bullseye-v20211209", image_name);
    }
}
