use std::fs;
use std::io::Write;
use std::path::PathBuf;

use Directory::*;

use crate::api::task;
use crate::api::util::{DataDir, ProjectDir};

pub struct Batch {}

impl Batch {
    pub fn write_all(files: Vec<&dyn File>) -> task::Result<()> {
        for file in files {
            file.write()?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub enum Directory {
    AnsibleRole { role_name: String },
    AnsibleRoles,
    GeneratedRoot,
    ImageArchive,
    ProjectRoot,
}

impl Directory {
    pub fn create_dir(&self) -> task::Result<PathBuf> {
        let path = self.path();
        fs::create_dir_all(&path)?;
        Ok(path)
    }

    pub fn join_path<S: AsRef<str>>(&self, filename: S) -> PathBuf {
        self.path().join(filename.as_ref())
    }

    pub fn create_sub_dir<S: AsRef<str>>(&self, path: S) -> task::Result<PathBuf> {
        let path = self.sub_path(path.as_ref());
        fs::create_dir_all(&path)?;
        Ok(path)
    }

    pub fn path(&self) -> PathBuf {
        match self {
            AnsibleRole { role_name } => AnsibleRoles.sub_path(role_name),
            AnsibleRoles => DataDir::path().join("roles"),
            GeneratedRoot => DataDir::path(),
            ImageArchive => DataDir::path().join("images"),
            ProjectRoot => ProjectDir::path(),
        }
    }

    pub fn sub_path<S: AsRef<str>>(&self, path: S) -> PathBuf {
        self.path().join(path.as_ref())
    }

    pub fn write<S: AsRef<str>>(&self, dir_path_opt: Option<S>, filename: S, content: S) -> task::Result<()> {
        self.write_bytes(dir_path_opt, filename, content.as_ref().as_bytes().to_vec())
    }

    pub fn write_bytes<S: AsRef<str>>(&self, dir_path_opt: Option<S>, filename: S, content: Vec<u8>) -> task::Result<()> {
        let dir_path = match dir_path_opt {
            None => task::Result::Ok(self.path()),
            Some(dir_path) => {
                let joined_dir_path = self.join_path(dir_path);
                fs::create_dir_all(&joined_dir_path)?;
                Ok(joined_dir_path)
            },
        }?;
        write_bytes(dir_path.join(filename.as_ref()), content)
    }
}

pub trait File {
    fn content(&self) -> task::Result<Vec<u8>>;

    fn output_path(&self) -> (Directory, String);

    fn write(&self) -> task::Result<()> {
        let (dir, filename) = self.output_path();
        write_bytes(dir.sub_path(filename), self.content()?)
    }
}

pub fn write<S: AsRef<str>>(p: PathBuf, content: S) -> task::Result<()> {
    write_bytes(p, content.as_ref().as_bytes().to_vec())
}

pub fn write_bytes(p: PathBuf, content: Vec<u8>) -> task::Result<()> {
    let mut file = fs::File::create(p)?;
    file.write_all(content.as_slice())?;
    task::SUCCESS
}
