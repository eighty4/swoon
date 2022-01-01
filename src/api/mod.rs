use crate::api::CloudPlatform::*;

pub mod command;
pub mod config;
pub mod context;
pub mod process;
pub mod task;
pub mod template;
pub mod util;

pub const DEFAULT_OS: OperatingSystem = OperatingSystem::Debian { version: 11 };

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum OperatingSystem {
    Debian { version: i32 },
    Ubuntu { version: f32, minimal: bool },
}

impl OperatingSystem {
    pub fn from_string(s: &str) -> task::Result<OperatingSystem> {
        match s {
            "debian" | "debian:11" | "debian:bullseye" => Ok(DEFAULT_OS),
            "debian:10" | "debian:buster" => Ok(OperatingSystem::Debian { version: 10 }),
            "debian:9" | "debian:stretch" => Ok(OperatingSystem::Debian { version: 9 }),
            "ubuntu" | "ubuntu:20.04" | "ubuntu:focal" => Ok(OperatingSystem::Ubuntu { version: 20.04, minimal: false }),
            "ubuntu:minimal" | "ubuntu:20.04:minimal" | "ubuntu:focal:minimal" => Ok(OperatingSystem::Ubuntu { version: 20.04, minimal: true }),
            "ubuntu:18.04" | "ubuntu:bionic" => Ok(OperatingSystem::Ubuntu { version: 18.04, minimal: false }),
            "ubuntu:18.04:minimal" | "ubuntu:bionic:minimal" => Ok(OperatingSystem::Ubuntu { version: 18.04, minimal: true }),
            &_ => task::Error::result("invalid gcp family string"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            OperatingSystem::Debian { version } => format!("debian:{}", version),
            OperatingSystem::Ubuntu { version, minimal } => if *minimal {
                format!("ubuntu:{}:minimal", version)
            } else {
                format!("ubuntu:{}", version)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::api::OperatingSystem;
    use crate::api::OperatingSystem::*;

    #[test]
    fn test_operating_system_from_string() {
        assert_eq!(OperatingSystem::from_string("debian"), Ok(Debian { version: 11 }));
        assert_eq!(OperatingSystem::from_string("debian:bullseye"), Ok(Debian { version: 11 }));
        assert_eq!(OperatingSystem::from_string("debian:11"), Ok(Debian { version: 11 }));

        assert_eq!(OperatingSystem::from_string("debian:buster"), Ok(Debian { version: 10 }));
        assert_eq!(OperatingSystem::from_string("debian:10"), Ok(Debian { version: 10 }));

        assert_eq!(OperatingSystem::from_string("debian:stretch"), Ok(Debian { version: 9 }));
        assert_eq!(OperatingSystem::from_string("debian:9"), Ok(Debian { version: 9 }));

        assert_eq!(OperatingSystem::from_string("ubuntu"), Ok(Ubuntu { version: 20.04, minimal: false }));
        assert_eq!(OperatingSystem::from_string("ubuntu:20.04"), Ok(Ubuntu { version: 20.04, minimal: false }));
        assert_eq!(OperatingSystem::from_string("ubuntu:focal"), Ok(Ubuntu { version: 20.04, minimal: false }));

        assert_eq!(OperatingSystem::from_string("ubuntu:minimal"), Ok(Ubuntu { version: 20.04, minimal: true }));
        assert_eq!(OperatingSystem::from_string("ubuntu:20.04:minimal"), Ok(Ubuntu { version: 20.04, minimal: true }));
        assert_eq!(OperatingSystem::from_string("ubuntu:focal:minimal"), Ok(Ubuntu { version: 20.04, minimal: true }));

        assert_eq!(OperatingSystem::from_string("ubuntu:18.04"), Ok(Ubuntu { version: 18.04, minimal: false }));
        assert_eq!(OperatingSystem::from_string("ubuntu:bionic"), Ok(Ubuntu { version: 18.04, minimal: false }));

        assert_eq!(OperatingSystem::from_string("ubuntu:18.04:minimal"), Ok(Ubuntu { version: 18.04, minimal: true }));
        assert_eq!(OperatingSystem::from_string("ubuntu:bionic:minimal"), Ok(Ubuntu { version: 18.04, minimal: true }));
    }

    #[test]
    fn test_operating_system_to_string() {
        assert_eq!("debian:11", Debian { version: 11 }.to_string());
        assert_eq!("debian:10", Debian { version: 10 }.to_string());
        assert_eq!("debian:9", Debian { version: 9 }.to_string());
        assert_eq!("ubuntu:18.04:minimal", Ubuntu { version: 18.04, minimal: true }.to_string());
        assert_eq!("ubuntu:20.04:minimal", Ubuntu { version: 20.04, minimal: true }.to_string());
        assert_eq!("ubuntu:18.04", Ubuntu { version: 18.04, minimal: false }.to_string());
        assert_eq!("ubuntu:20.04", Ubuntu { version: 20.04, minimal: false }.to_string());
    }
}
