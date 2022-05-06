use crate::api::config::SwoonConfig;
use crate::api::OperatingSystem;
use crate::api::output::template::{Template, template_object};
use crate::packer::PackerSource;
use crate::platforms::gcloud::{GcloudContext, images};

enum SourceImageMethod {
    Family(String),
    Name(String),
}

impl SourceImageMethod {
    fn from_os(os: &OperatingSystem) -> Self {
        SourceImageMethod::Family(images::family_name_by_os(os))
    }
}

pub struct GcloudPackerSource {
    image_name: String,
    project_id: String,
    source_label: String,
    source_method: SourceImageMethod,
}

impl GcloudPackerSource {
    pub fn from_os(cfg: &SwoonConfig, gcloud_ctx: &GcloudContext, os: &OperatingSystem) -> Box<Self> {
        Self::new(cfg, gcloud_ctx, SourceImageMethod::from_os(os))
    }

    fn new(cfg: &SwoonConfig,
           gcloud_ctx: &GcloudContext,
           source_method: SourceImageMethod) -> Box<Self> {
        Box::from(Self {
            image_name: format!("{}-archetype-{}", cfg.org_name, "2020-01-02").to_string(),
            project_id: gcloud_ctx.default_project_id.clone(),
            source_label: "archetype".to_string(),
            source_method,
        })
    }
}

impl PackerSource for GcloudPackerSource {
    fn name(&self) -> String {
        format!("source.googlecompute.{}", self.source_label)
    }

    fn to_hcl(&self) -> String {
        let (source_image_method, source_image_value) = match &self.source_method {
            SourceImageMethod::Family(v) => ("source_image_family", v),
            SourceImageMethod::Name(v) => ("source_image", v),
        };
        let result = Template::render(
            include_bytes!("source.pkr.hcl.liquid"),
            &template_object!({
                "project_id": self.project_id,
                "source_image_method": source_image_method,
                "source_image_value": source_image_value,
                "image_name": self.image_name,
                "source_label": self.source_label,
            }),
        );
        match result {
            Ok(s) => s,
            Err(e) => e.exit(),
        }
    }
}
