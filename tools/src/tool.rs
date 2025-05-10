use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use std::os::unix::fs::symlink;

use crate::utils;

pub type ToolFn = fn() -> Result<(), Box<dyn error::Error>>;

const BIN_DIR: &str = "~/.val/bin";

#[derive(Clone)]
pub struct Tool {
    name: &'static str,
    func: ToolFn,
}

impl Tool {
    #[inline]
    pub fn install_dir() -> PathBuf {
        bin_dir()
    }

    #[inline]
    pub fn exe_path(profile: &str) -> PathBuf {
        exe_path(profile)
    }

    #[inline]
    pub const fn new(name: &'static str, func: ToolFn) -> Self {
        Self { name, func }
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.name
    }

    #[inline]
    pub fn func(&self) -> ToolFn {
        self.func
    }

    #[inline]
    pub fn call(&self) -> Result<(), Box<dyn error::Error>> {
        (self.func)()
    }

    pub fn install_path(&self) -> PathBuf {
        Self::install_dir().join(&self.name)
    }

    pub fn install_debug_path(&self) -> PathBuf {
        let name = format!("_{}", self.name);
        Self::install_dir().join(name)
    }

    /// Warning: This install a symlink to the release binary,
    /// so if this is executed in a debug build, then the symlink
    /// is still installed. However, it will first work after
    /// executing a release build.
    pub fn install(&self) -> io::Result<()> {
        // TODO: On Windows copy and rename the binary instead

        let link = self.install_path();
        let exe = Self::exe_path("release");

        println!("Installing `{}`", link.display());
        symlink(&exe, &link)?;

        Ok(())
    }

    pub fn debug_script(&self) -> String {
        let tool_name = &self.name;

        let manifest_path = env!("CARGO_MANIFEST_PATH");
        let manifest_path = shlex::try_quote(manifest_path).unwrap();

        let exe = Self::exe_path("debug");
        let exe = exe.to_str().unwrap();
        let exe = shlex::try_quote(exe).unwrap();

        [
            "#!/usr/bin/env bash",
            "set -e",
            &format!("cargo build --manifest-path {manifest_path}"),
            &format!("# {tool_name} \"$@\""),
            &format!("exec -a \"{tool_name}\" {exe} \"$@\""),
        ]
        .join("\n")
    }

    pub fn install_debug(&self) -> io::Result<()> {
        let path = self.install_debug_path();
        let code = self.debug_script();

        println!("Installing `{}`", path.display());
        fs::write(&path, code).unwrap();

        utils::mark_executable(&path)?;

        Ok(())
    }

    pub fn uninstall(&self) -> io::Result<()> {
        let path = self.install_path();

        // TODO: Check if symlink actually points to `tools` exe
        // TODO: Might cause an issue if `tools` has been used
        if utils::try_remove_file(&path)? {
            println!("Removed `{}`", path.display());
        }

        Ok(())
    }

    pub fn uninstall_debug(&self) -> io::Result<()> {
        let path = self.install_debug_path();

        // TODO: Check if symlink actually points to `tools` exe
        // TODO: Might cause an issue if `tools` has been used
        if utils::try_remove_file(&path)? {
            println!("Removed `{}`", path.display());
        }

        Ok(())
    }
}

impl fmt::Debug for Tool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tool")
            .field("name", &self.name)
            .field("func", &self.func)
            .field("install_path", &self.install_path())
            .field("install_debug_path", &self.install_debug_path())
            .finish()
    }
}

fn bin_dir() -> PathBuf {
    let bin_dir = shellexpand::tilde(BIN_DIR);
    PathBuf::from(bin_dir.as_ref())
}

fn exe_path(profile: &str) -> PathBuf {
    // Not using `std::env::current_exe()` as executing `reinstall-tools`
    // results in the wrong executable path
    dotfiles_dir()
        .join("target")
        .join(profile)
        .join(env!("CARGO_BIN_NAME"))
}

fn dotfiles_dir() -> &'static Path {
    let tools_dir = manifest_dir();
    tools_dir.parent().unwrap()
}

#[inline]
fn manifest_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}
