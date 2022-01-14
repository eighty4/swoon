use crate::api::{CloudPlatform, OperatingSystem, task};
use crate::SwoonContext;

pub enum ImageSource {
    OperatingSystem { os: OperatingSystem },
    Image { name: String },
}

pub struct ImageSpec {
    pub platform: CloudPlatform,
    pub source: ImageSource,
}

impl ImageSpec {
    pub fn source_string(&self) -> String {
        match &self.source {
            ImageSource::OperatingSystem { os } => os.to_string(),
            ImageSource::Image { name } => name.clone(),
        }
    }
}

pub struct BakingPlan {
    pub default_archetype: ImageSpec,
}

impl BakingPlan {
    pub fn from(ctx: &SwoonContext) -> task::Result<Self> {
        let cfg = ctx.config.as_ref().expect("no config");
        Ok(BakingPlan {
            default_archetype: ImageSpec {
                source: ImageSource::OperatingSystem { os: cfg.default_os.clone() },
                platform: cfg.default_platform.clone(),
            }
        })
    }
}

// .swoon/images/{platform}/archetypes/{os}/{image_name}/{files}
// .swoon/images/gcp/archetypes/debian_11/debian_11-archetype-2022-01-04/{files}
// .swoon/images/gcp/archetypes/ubuntu_2004_minimal/debian_11-archetype-2022-01-04/{files}

// .swoon/images/{platform}/instances/{instance_label}/{image_name}/{files}
// .swoon/images/gcp/instances/postgres/{instance_label}/postgres-2022-01-04/{files}
