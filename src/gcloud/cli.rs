use std::process::{Command, Stdio};

use crate::api::context::{BinaryPaths, SwoonContext};
use crate::api::SwoonError;
use crate::gcloud::cli::GcpImageFamily::Debian;
use crate::swoon_error_result;

pub enum GcpImageFamily {
    Debian { version: i32 },
    Ubuntu { version: i32, minimal: bool },
}

pub const DEFAULT_OS: GcpImageFamily = Debian { version: 11 };

impl GcpImageFamily {
    pub fn from_string(s: &str) -> Result<GcpImageFamily, SwoonError> {
        match s {
            "debian" | "debian:11" | "debian:bullseye" => Result::Ok(DEFAULT_OS),
            "debian:10" | "debian:buster" => Result::Ok(GcpImageFamily::Debian { version: 10 }),
            "debian:9" | "debian:stretch" => Result::Ok(GcpImageFamily::Debian { version: 9 }),
            "ubuntu" | "ubuntu:20.04" | "ubuntu:focal" => Result::Ok(GcpImageFamily::Ubuntu { version: 2004, minimal: false }),
            "ubuntu:minimal" | "ubuntu:20.04:minimal" | "ubuntu:focal:minimal" => Result::Ok(GcpImageFamily::Ubuntu { version: 2004, minimal: true }),
            "ubuntu:18.04" | "ubuntu:bionic" => Result::Ok(GcpImageFamily::Ubuntu { version: 1804, minimal: false }),
            "ubuntu:18.04:minimal" | "ubuntu:bionic:minimal" => Result::Ok(GcpImageFamily::Ubuntu { version: 1804, minimal: true }),
            &_ => swoon_error_result("invalid gcp image family string")
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            GcpImageFamily::Debian { version } => format!("debian-{}", version),
            GcpImageFamily::Ubuntu { version, minimal } => if *minimal {
                format!("ubuntu-minimal-{}-lts", version)
            } else {
                format!("ubuntu-{}-lts", version)
            }
        }
    }
}

pub fn all_image_names(ctx: &SwoonContext) -> Result<Vec<String>, SwoonError> {
    let output = Command::new(ctx.gcloud_bin_path())
        .args(["compute", "images", "list",
            "--no-standard-images",
            "--format=value(NAME)",
            format!("--filter=name:{}-{}-", "eighty4", "archetype").as_ref(),
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to execute gcloud compute images list");
    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stderr));
        return swoon_error_result("error running gcloud::cli::all_image_names");
    }
    let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    return if output_str.len() == 0 {
        Result::Ok(Vec::new())
    } else {
        Result::Ok(output_str.split("\n").map(|s| s.to_string()).collect())
    };
}

pub fn default_project_id(ctx: &SwoonContext) -> Result<String, SwoonError> {
    let output = Command::new(ctx.gcloud_bin_path())
        .args(["config", "get-value", "core/project"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to execute gcloud config get-value core/project");
    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stderr));
        return swoon_error_result("error running gcloud::cli::default_project_id");
    }
    return Result::Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
}

pub fn image_name_by_family(ctx: &SwoonContext, image: GcpImageFamily) -> Result<String, SwoonError> {
    let output = Command::new(ctx.gcloud_bin_path())
        .args(["compute", "images", "list",
            "--format=value(NAME)",
            format!("--filter=family:{}", image.to_string()).as_ref(),
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to execute gcloud compute images list");
    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stderr));
        return swoon_error_result("error running gcloud::cli::image_name_by_family");
    }
    return Result::Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
}

#[cfg(test)]
mod tests {
    use crate::gcloud::cli::GcpImageFamily::*;
    use crate::init_swoon_context;

    use super::*;

    #[test]
    fn test_all_image_names() {
        let ctx = init_swoon_context().unwrap();
        let image_names = all_image_names(&ctx).unwrap();
        assert_eq!(1, image_names.len());
    }

    #[test]
    fn test_default_project_id() {
        let ctx = init_swoon_context().unwrap();
        let project_id = default_project_id(&ctx).unwrap();
        assert_eq!("example-project-id", project_id);
    }

    #[test]
    fn test_image_family_name() {
        assert_eq!("debian-11", Debian { version: 11 }.to_string());
        assert_eq!("debian-10", Debian { version: 10 }.to_string());
        assert_eq!("debian-9", Debian { version: 9 }.to_string());
        assert_eq!("ubuntu-minimal-1804-lts", Ubuntu { version: 1804, minimal: true }.to_string());
        assert_eq!("ubuntu-minimal-2004-lts", Ubuntu { version: 2004, minimal: true }.to_string());
        assert_eq!("ubuntu-1804-lts", Ubuntu { version: 1804, minimal: false }.to_string());
        assert_eq!("ubuntu-2004-lts", Ubuntu { version: 2004, minimal: false }.to_string());
    }

    #[test]
    fn test_image_name_by_family() {
        let ctx = init_swoon_context().unwrap();
        let image_name = image_name_by_family(&ctx, Debian { version: 11 }).unwrap();
        assert_eq!("debian-11-bullseye-v20211209", image_name);
    }
}
