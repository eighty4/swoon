use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::api::task;

pub struct DataDir {}

impl DataDir {
    const DEFAULT_PATH: &'static str = "./.swoon";

    pub fn create_sub_dir(path: &str) -> task::Result<PathBuf> {
        let path = Self::sub_dir_path(path);
        fs::create_dir_all(&path)?;
        Ok(path)
    }

    pub fn init() -> task::Result<()> {
        fs::create_dir_all(Self::sub_dir_path("images"))?;
        task::SUCCESS
    }

    pub fn path() -> PathBuf {
        PathBuf::from(Self::DEFAULT_PATH)
    }

    pub fn sub_dir_path(path: &str) -> PathBuf {
        Self::path().join(path)
    }
}

pub fn split_string(split: &str, string: String) -> Vec<String> {
    if string.is_empty() {
        vec!()
    } else {
        string.split(split).map(|s| s.to_string()).collect()
    }
}

pub fn write_file(path: &PathBuf, content: String) -> task::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    task::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_string() {
        let result = split_string(" ", String::from("happy new year"));
        assert_eq!(result, vec!("happy".to_string(), "new".to_string(), "year".to_string()));
    }

    #[test]
    fn test_split_string_with_empty_input() {
        let result = split_string(" ", String::from(""));
        assert_eq!(result, Vec::<String>::new());
    }
}
