use std::hash::{Hash, Hasher};

use crate::api::CloudPlatform::*;

pub mod binaries;
pub mod command;
pub mod config;
pub mod context;
pub mod process;
pub mod task;
pub mod template;
pub mod util;

pub const DEBIAN_11: OperatingSystem = OperatingSystem::Debian { version: 11 };
pub const DEBIAN_10: OperatingSystem = OperatingSystem::Debian { version: 10 };
pub const DEBIAN_9: OperatingSystem = OperatingSystem::Debian { version: 9 };

pub const UBUNTU_2004: OperatingSystem = OperatingSystem::Ubuntu {
    version: MajorMinorVersion { major: 20, minor: 04 },
    minimal: false,
};
pub const UBUNTU_2004_MINIMAL: OperatingSystem = OperatingSystem::Ubuntu {
    version: MajorMinorVersion { major: 20, minor: 04 },
    minimal: true,
};
pub const UBUNTU_1804: OperatingSystem = OperatingSystem::Ubuntu {
    version: MajorMinorVersion { major: 18, minor: 04 },
    minimal: false,
};
pub const UBUNTU_1804_MINIMAL: OperatingSystem = OperatingSystem::Ubuntu {
    version: MajorMinorVersion { major: 18, minor: 04 },
    minimal: true,
};

pub const DEFAULT_OS: OperatingSystem = DEBIAN_11;

#[derive(Clone, Debug, PartialEq)]
pub enum CloudPlatform {
    AWS,
    Azure,
    DigitalOcean,
    GCP,
    Linode,
    Vultr,
}

impl CloudPlatform {
    pub fn from_str(label: &str) -> Option<Self> {
        match label {
            "aws" => Some(AWS),
            "azure" => Some(Azure),
            "digitalocean" => Some(DigitalOcean),
            "gcp" => Some(GCP),
            "linode" => Some(Linode),
            "vultr" => Some(Vultr),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            AWS => "aws",
            Azure => "azure",
            DigitalOcean => "digitalocean",
            GCP => "gcp",
            Linode => "linode",
            Vultr => "vultr",
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MajorMinorVersion {
    pub major: i32,
    pub minor: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OperatingSystem {
    Debian { version: i32 },
    Ubuntu { version: MajorMinorVersion, minimal: bool },
}

impl OperatingSystem {
    pub fn from_string(s: &str) -> task::Result<OperatingSystem> {
        match s {
            "debian" | "debian:11" | "debian:bullseye" => Ok(DEBIAN_11),
            "debian:10" | "debian:buster" => Ok(DEBIAN_10),
            "debian:9" | "debian:stretch" => Ok(DEBIAN_9),
            "ubuntu" | "ubuntu:20.04" | "ubuntu:focal" => Ok(UBUNTU_2004),
            "ubuntu:minimal" | "ubuntu:20.04:minimal" | "ubuntu:focal:minimal" => Ok(UBUNTU_2004_MINIMAL),
            "ubuntu:18.04" | "ubuntu:bionic" => Ok(UBUNTU_1804),
            "ubuntu:18.04:minimal" | "ubuntu:bionic:minimal" => Ok(UBUNTU_1804_MINIMAL),
            &_ => task::Error::result("invalid gcp family string"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            OperatingSystem::Debian { version } => format!("debian:{}", version),
            OperatingSystem::Ubuntu { version, minimal } => {
                if *minimal {
                    format!("ubuntu:{}.{:02}:minimal", version.major, version.minor)
                } else {
                    format!("ubuntu:{}.{:02}", version.major, version.minor)
                }
            }
        }
    }
}

impl Hash for OperatingSystem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            OperatingSystem::Debian { version } => version.hash(state),
            OperatingSystem::Ubuntu { version, minimal } => {
                version.hash(state);
                minimal.hash(state);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_operating_system_from_string() {
        assert_eq!(OperatingSystem::from_string("debian"), Ok(DEBIAN_11));
        assert_eq!(OperatingSystem::from_string("debian:bullseye"), Ok(DEBIAN_11));
        assert_eq!(OperatingSystem::from_string("debian:11"), Ok(DEBIAN_11));

        assert_eq!(OperatingSystem::from_string("debian:buster"), Ok(DEBIAN_10));
        assert_eq!(OperatingSystem::from_string("debian:10"), Ok(DEBIAN_10));

        assert_eq!(OperatingSystem::from_string("debian:stretch"), Ok(DEBIAN_9));
        assert_eq!(OperatingSystem::from_string("debian:9"), Ok(DEBIAN_9));

        assert_eq!(OperatingSystem::from_string("ubuntu"), Ok(UBUNTU_2004));
        assert_eq!(OperatingSystem::from_string("ubuntu:20.04"), Ok(UBUNTU_2004));
        assert_eq!(OperatingSystem::from_string("ubuntu:focal"), Ok(UBUNTU_2004));

        assert_eq!(OperatingSystem::from_string("ubuntu:minimal"), Ok(UBUNTU_2004_MINIMAL));
        assert_eq!(OperatingSystem::from_string("ubuntu:20.04:minimal"), Ok(UBUNTU_2004_MINIMAL));
        assert_eq!(OperatingSystem::from_string("ubuntu:focal:minimal"), Ok(UBUNTU_2004_MINIMAL));

        assert_eq!(OperatingSystem::from_string("ubuntu:18.04"), Ok(UBUNTU_1804));
        assert_eq!(OperatingSystem::from_string("ubuntu:bionic"), Ok(UBUNTU_1804));

        assert_eq!(OperatingSystem::from_string("ubuntu:18.04:minimal"), Ok(UBUNTU_1804_MINIMAL));
        assert_eq!(OperatingSystem::from_string("ubuntu:bionic:minimal"), Ok(UBUNTU_1804_MINIMAL));
    }

    #[test]
    fn test_operating_system_to_string() {
        assert_eq!("debian:11", DEBIAN_11.to_string());
        assert_eq!("debian:10", DEBIAN_10.to_string());
        assert_eq!("debian:9", DEBIAN_9.to_string());
        assert_eq!("ubuntu:18.04:minimal", UBUNTU_1804_MINIMAL.to_string());
        assert_eq!("ubuntu:20.04:minimal", UBUNTU_2004_MINIMAL.to_string());
        assert_eq!("ubuntu:18.04", UBUNTU_1804.to_string());
        assert_eq!("ubuntu:20.04", UBUNTU_2004.to_string());
    }
}
