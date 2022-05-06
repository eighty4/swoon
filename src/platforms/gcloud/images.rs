use crate::api::OperatingSystem;

pub fn family_name_by_os(os: &OperatingSystem) -> String {
    match os {
        OperatingSystem::Debian { version } => format!("debian-{}", version),
        OperatingSystem::Ubuntu { version, minimal } => {
            let fam_prefix = if *minimal { "ubuntu-minimal" } else { "ubuntu" };
            format!("{}-{:02}{:02}-lts", fam_prefix, version.major, version.minor)
        }
    }
}
