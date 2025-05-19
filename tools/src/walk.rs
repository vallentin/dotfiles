use std::cmp::Reverse;
use std::env;
use std::error;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::utils;
use crate::utils::ansi::{BRIGHT_BLACK, BRIGHT_CYAN, GREEN, MAGENTA, RED, RESET};

#[rustfmt::skip]
const IGNORED_DIRS: &[&str] = &[
    ".git",
    ".build",
    "target",
    "env",
    "node_modules",
];

pub fn run() -> Result<(), Box<dyn error::Error>> {
    let mut dirs = env::args().skip(1).map(PathBuf::from).collect::<Vec<_>>();

    if dirs.is_empty() {
        let cwd = env::current_dir()?;
        dirs.push(cwd);
    }

    let walker = Walker::new();

    for dir in dirs {
        walker.walk_dir(dir)?;
    }

    Ok(())
}

struct Walker<'a> {
    ignored_dirs: &'a [&'a str],
    ignore_hidden: bool,
}

impl Walker<'static> {
    pub fn new() -> Self {
        Self {
            ignored_dirs: IGNORED_DIRS,
            ignore_hidden: false,
        }
    }
}

impl Walker<'_> {
    pub fn walk_dir(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let path = path.as_ref();

        let read_dir = match fs::read_dir(path) {
            Ok(read_dir) => read_dir,
            Err(err) => {
                eprintln!("{RED}Error: {err}{RESET}");
                return Ok(());
            }
        };

        let mut entries = read_dir
            .into_iter()
            .map(|entry| {
                let entry = entry?;
                let path = entry.path();
                let file_type = entry.file_type()?;
                Ok((path, file_type))
            })
            .filter_map(io::Result::ok)
            .collect::<Vec<_>>();

        entries.sort_by(|a, b| a.0.cmp(&b.0));
        entries.sort_by_key(|(_path, file_type)| Reverse(file_type.is_dir()));

        for (path, file_type) in entries {
            let is_symlink = file_type.is_symlink();
            let mut is_dir = file_type.is_dir();

            let file_name = path.file_name().and_then(OsStr::to_str);
            let is_path_ignored = file_name.map_or(false, |file_name| {
                if self.ignored_dirs.contains(&file_name) {
                    return true;
                }

                if self.ignore_hidden && file_name.starts_with('.') {
                    return true;
                }

                false
            });

            let color = match () {
                _ if is_dir => BRIGHT_CYAN,
                _ if is_symlink => MAGENTA,
                _ => match utils::is_executable(&path) {
                    Ok(executable) if executable => GREEN,
                    _ => RESET,
                },
            };

            if is_symlink {
                let real_path = path.read_link()?;
                is_dir = real_path.metadata()?.is_dir();
            }

            let path_end = if is_dir { "/" } else { "" };

            print!("{color}{}{path_end}{RESET}", path.display());

            if is_symlink {
                let real_path = path.read_link()?;
                print!("{BRIGHT_BLACK} -> {}{path_end}{RESET}", real_path.display());
            }

            println!();

            if is_dir && !is_path_ignored {
                self.walk_dir(path)?;
            }
        }
        Ok(())
    }
}
