use crate::api::context::SwoonContext;
use crate::api::output::file::{Directory, File};
use crate::api::output::file::Directory::GeneratedRoot;
use crate::api::output::template::{Template, template_object};
use crate::api::task;
pub use crate::platforms::packer::source;

pub trait PackerSource {
    fn name(&self) -> String;
    fn to_hcl(&self) -> String;
}

pub struct PackerBuild {
    // provisioning: PackerProvisioning,
    sources: Vec<Box<dyn PackerSource>>,
}

impl PackerBuild {
    pub fn archetype(sources: Vec<Box<dyn PackerSource>>) -> Self {
        Self {
            // provisioning: PackerProvisioning::archetype(),
            sources,
        }
    }

    pub fn default_archetype(ctx: &SwoonContext) -> Self {
        let cfg = ctx.config();
        Self::archetype(vec!(
            source::from_os(ctx, &cfg.default_platform, &cfg.default_os)
        ))
    }

    pub fn bake(&self) -> task::Result<()> {
        // todo[bake-archetype] invoke packer
        Ok(())
    }

    pub(crate) fn to_hcl(&self) -> String {
        let source = self.sources.get(0).unwrap().as_ref();

        let result = Template::render(
            include_bytes!("build.pkr.hcl.liquid"),
            &template_object!({
                "source_name": source.name(), // todo[bake-archetype] handle sources as a list
                "image_name": "archetype", // todo[bake-archetype] resolve image name
            }),
        );
        match result {
            Ok(s) => s,
            Err(e) => e.exit(),
        }
    }

    pub fn write_config_files(&self) -> task::Result<()> {
        PackerBuildFile::example(self).write()
    }
}

// pub struct PackerProvisioning {
//     playbook: Option<AnsiblePlaybook>,
//     startup: Option<String>,
// }
//
// impl PackerProvisioning {
//     pub fn default() -> Self {
//         Self {
//             playbook: None,
//             startup: None,
//         }
//     }
//
//     pub fn archetype() -> Self {
//         Self {
//             playbook: Some(AnsiblePlaybook::archetype()),
//             startup: None,
//         }
//     }
// }

pub struct PackerBuildFile<'a> {
    build: &'a PackerBuild,
}

impl<'a> PackerBuildFile<'a> {
    pub fn example(build: &'a PackerBuild) -> Self {
        Self {
            build,
        }
    }
}

impl<'a> File for PackerBuildFile<'a> {
    fn content(&self) -> task::Result<Vec<u8>> {
        let sources_hcl = self.build.sources.iter()
            .map(|source| { source.to_hcl() })
            .collect::<Vec<String>>()
            .join("\n\n");

        let build_hcl = self.build.to_hcl();

        Ok(format!("{}\n\n{}", sources_hcl, build_hcl).as_bytes().to_vec())
    }

    fn output_path(&self) -> (Directory, String) {
        (GeneratedRoot, "archetype.pkr.hcl".to_string())
    }
}
