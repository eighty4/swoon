use std::path::PathBuf;

use clap::ArgMatches;
use console::Term;

use crate::api::binaries::{BinaryName, BinaryPaths};
pub use crate::api::binaries::PathLookup;
use crate::api::config::SwoonConfig;
use crate::api::task;
use crate::api::util::error_exit;
use crate::platforms::PlatformContexts;

#[derive(Clone)]
pub struct SwoonOpts {
    pub(crate) debug: bool,
}

pub struct SwoonContext {
    binary_paths: BinaryPaths,
    pub config: Option<SwoonConfig>,
    pub opts: SwoonOpts,
    pub platforms: PlatformContexts,
    terminal: Term,
}

impl SwoonContext {
    #[cfg(test)]
    pub fn default() -> task::Result<Self> {
        Self::init(SwoonOpts {
            debug: false,
        })
    }

    pub fn init_from_args(args: &ArgMatches) -> task::Result<Self> {
        Self::init(SwoonOpts {
            debug: args.is_present("debug"),
        })
    }

    pub fn init(opts: SwoonOpts) -> task::Result<Self> {
        let binary_paths = BinaryPaths::init();
        let config = SwoonConfig::read_from_current_dir()?;
        let platforms = PlatformContexts::init(&binary_paths, &config);
        Ok(SwoonContext {
            binary_paths,
            config,
            opts,
            platforms,
            terminal: Term::stdout(),
        })
    }

    pub fn with_config(&self, new_config: SwoonConfig) -> Self {
        let config = Some(new_config);
        let platforms = PlatformContexts::init(&self.binary_paths, &config);
        Self {
            binary_paths: self.binary_paths.clone(),
            config,
            opts: self.opts.clone(),
            platforms,
            terminal: Term::stdout(),
        }
    }

    pub fn write_line<S: AsRef<str>>(&self, s: S) {
        let result = self.terminal.write_line(s.as_ref());
        if let Err(e) = result {
            error_exit(&e);
        }
    }
}

impl PathLookup for SwoonContext {
    fn lookup(&self, bin: BinaryName) -> PathBuf {
        self.binary_paths.lookup(bin)
    }
}
