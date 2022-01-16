use std::env;
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
        Self::invoke_from_dir(env::current_dir()?, cmd, args)
    }

    pub fn invoke_from_dir<I, S>(invoke_dir: PathBuf, cmd: &PathBuf, args: I) -> task::Result<String>
        where
            I: IntoIterator<Item=S>,
            S: AsRef<OsStr>, {
        let mut debug_print_cmd = String::from(cmd.file_name().unwrap().to_str().unwrap());
        let mut vec_copy_args = Vec::new();
        args.into_iter().for_each(|s| {
            debug_print_cmd.push_str(format!(" {}", s.as_ref().to_str().unwrap()).as_str());
            vec_copy_args.push(s);
        });

        let output = Command::new(cmd)
            .current_dir(invoke_dir)
            .args(vec_copy_args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            let exit_code = output.status.code()
                .map_or(String::from("?"), |c| c.to_string());
            println!("\n{}\nexit code {}\ncommand output:\n\n{}",
                     debug_print_cmd,
                     exit_code,
                     String::from_utf8_lossy(&output.stderr),
            );

            let error_msg = format!("exit code {} invoking {}", exit_code, debug_print_cmd);
            task::Error::result(error_msg)
        }
    }
}
