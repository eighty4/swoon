pub mod source {
    use crate::api::{CloudPlatform, OperatingSystem, task};
    use crate::packer::PackerSource;
    use crate::platforms::gcloud::packer::GcloudPackerSource;
    use crate::SwoonContext;

    fn not_yet_implemented(platform: &CloudPlatform) -> ! {
        task::fatal(format!(
            "unable to create packer source for unimplemented platform {}",
            platform.to_str())
        )
    }

    pub fn from_os(ctx: &SwoonContext, platform: &CloudPlatform, os: &OperatingSystem) -> Box<dyn PackerSource> {
        match platform {
            CloudPlatform::GCP => GcloudPackerSource::from_os(ctx.config(), ctx.platforms.gcloud_ctx(), os),
            _ => not_yet_implemented(platform),
        }
    }
}
