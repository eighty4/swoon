use std::collections::HashMap;
use std::path::PathBuf;

use which::which;

use crate::api::binaries::BinaryName::*;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum BinaryName {
    // Ansible,
    // Aws,
    // Azure,
    // Docker,
    Gcloud,
    // Packer,
    // Terraform,
}

impl BinaryName {
    pub fn all() -> Vec<Self> {
        [
            // Ansible,
            // Aws,
            // Azure,
            // Docker,
            Gcloud,
            // Packer,
            // Terraform,
        ].to_vec()
    }

    pub fn filename(&self) -> String {
        String::from(match &self {
            // Ansible => "ansible",
            // Aws => "aws",
            // Azure => "?",
            // Docker => "docker",
            Gcloud => "gcloud",
            // Packer => "packer",
            // Terraform => "terraform",
        })
    }
}

pub trait PathLookup {
    // fn ansible_path(&self) -> PathBuf {
    //     self.lookup(Ansible)
    // }

    // fn aws_path(&self) -> PathBuf {
    //     self.lookup(Aws)
    // }

    // fn azure_path(&self) -> PathBuf {
    //     self.lookup(Azure)
    // }

    // fn docker_path(&self) -> PathBuf {
    //     self.lookup(Docker)
    // }

    fn gcloud_path(&self) -> PathBuf {
        self.lookup(Gcloud)
    }

    fn lookup(&self, bin: BinaryName) -> PathBuf;

    // fn packer_path(&self) -> PathBuf {
    //     self.lookup(Packer)
    // }

    // fn terraform_path(&self) -> PathBuf {
    //     self.lookup(Terraform)
    // }
}

#[derive(Clone)]
pub struct BinaryPaths {
    paths: HashMap<BinaryName, PathBuf>,
}

impl BinaryPaths {
    pub fn init() -> Self {
        let mut paths = HashMap::new();
        BinaryName::all().iter().for_each(|b| {
            let path = which(b.filename());
            if path.is_ok() {
                paths.insert(b.clone(), path.unwrap());
            }
        });

        Self { paths }
    }
}

impl PathLookup for BinaryPaths {
    fn lookup(&self, bin: BinaryName) -> PathBuf {
        self.paths.get(&bin).expect("asdf").clone()
    }
}
