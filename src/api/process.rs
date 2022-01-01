use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::api::task;

pub struct Process {}

impl Process {
    pub fn invoke<I, S>(cmd: &PathBuf, args: I) -> task::Result<String>
        where
            I: IntoIterator<Item=S>,
            S: AsRef<OsStr>, {
        let output = Command::new(cmd)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            println!("{}", String::from_utf8_lossy(&output.stderr));
            let exit_code = output.status.code().map_or(String::from("?"), |c| c.to_string());
            let error_msg = format!("exit code {} invoking {} process",
                                    exit_code, cmd.to_str().unwrap());
            task::Error::result(error_msg.as_str())
        }
    }
}
